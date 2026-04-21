# Write Path Strategy — Decisions & Rationale

This document captures the architectural decisions made for the CQRS command (write) path in this project. Use it as a reference when implementing new commands for any entity.

---

## 1. The "Insert Struct" Pattern (Option C)

**Decision**: The domain produces a fully-resolved "insert" struct. The write port accepts that struct, not raw fields or the creation payload.

**Why we chose this over alternatives**:

| Rejected Option | Why |
|---|---|
| Entity constructor with `id: 0` | Not an established sqlx pattern. The domain entity would hold invalid state (fake id, fabricated timestamps). Persistence would need to silently ignore fields — implicit contract, not enforced by types. |
| `Option<id>` on the entity | Actively discouraged in the Rust/sqlx ecosystem. Pollutes every downstream usage with unwrap/match. |
| Same entity for insert and read | sqlx's `query_as!` maps struct fields to SQL columns. An insert struct shouldn't have `id`, `ident`, `dt_created` — those are DB-assigned. Separate structs is the dominant sqlx pattern (see launchbadge/realworld-axum-sqlx). |

**The pattern**:

```
ContactInquiryCreationPayload  (caller-provided fields only)
        │
        │  payload.prepare(spam_likelihood)
        ▼
ContactInquiryPrepared           (all fields needed for INSERT, including domain-resolved status)
        │
        │  write_port.create_contact_inquiry(&prepared)
        ▼
ContactInquiry                 (full entity with DB-assigned id, ident, timestamps)
```

The `INSERT ... RETURNING *` SQL bridges the insert struct → entity. You send a `ContactInquiryPrepared`, bind its fields, and map the returned row to `ContactInquiry` via `ContactInquiryDbRow`.

---

## 2. Status Assignment is a Domain Decision

**Decision**: The domain — not the application, not the caller — decides the initial status.

**Reasoning**: "What status does a new inquiry start with?" is a business rule. The application layer orchestrates (calls spam check, calls `prepare()`), but the domain method `prepare()` contains the `if is_spam { "spam" } else { "unread" }` logic.

**Flow**:
1. Application gets the `ContactInquiryCreationPayload` (from HTTP layer)
2. Application calls `spam_service.get_spam_likelihood(&payload.message)` → `u8`
3. Application calls `payload.prepare(spam_likelihood)` → `ContactInquiryPrepared`
4. Inside `prepare()`, the domain calls `self.is_spam(likelihood)` and maps `bool → status`
5. Application passes `&prepared` to the write port — status is already resolved

The application never contains status string literals. It doesn't know "unread" or "spam" exist. It just passes data through.

---

## 3. `prepare()` Consumes the Payload

**Decision**: `prepare(self, ...)` takes ownership (move), not `&self` (borrow).

**Why**: The payload's job is done after `prepare()`. Moving fields into the insert struct avoids cloning every String. The compiler prevents accidental reuse of a consumed payload. This is Rust's ownership system enforcing single-use semantics at zero cost.

---

## 4. Spam Likelihood as a Percentage

**Decision**: The spam scoring port returns a `u8` representing 0–100% likelihood of spam. Threshold is 30%.

**Why 0–100 percentage**:
- Unambiguous direction (0 = not spam, 100 = definitely spam)
- LLM-prompt-friendly ("Return a single integer 0–100")
- `u8` fits 0–255, so 0–100 sits comfortably within the type
- Human-readable in logs and debugging

**Why 30% threshold**:
- Low enough to catch obvious spam
- High enough to avoid false positives on legitimate messages with URLs or salesy language
- Can be tuned later by changing one constant — no signature changes needed

**Validation**: The adapter (persistence layer) is responsible for validating the LLM response is parseable and within 0–100. Out-of-range values should return `Err`, not be clamped — garbage in should be caught at the boundary.

---

## 5. Commands Struct Pattern

**Decision**: Each entity's write use cases live in a `Commands` struct that holds `Arc<dyn WriteUnitOfWorkFactory>` plus any service ports needed.

```rust
pub struct ContactInquiryCommands {
    pub uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
    pub spam_service: Arc<dyn ContactInquirySpamRatingPort>,
}
```

**Why**:
- Mirrors the `Queries` struct pattern on the read side
- The handler builds this struct from `deps.write_uow_factory` and `deps.services`
- Each method call (e.g., `commands.create(payload)`) builds a fresh UoW internally
- Commands never touch `deps.read_repos` — that's the ro_pool query path

**Comparison to Python**:

```
Python                                  Rust
──────                                  ────
class AuthorWriteService:               pub struct ContactInquiryCommands
    def __init__(self, uow):                uow_factory: Arc<dyn WriteUnitOfWorkFactory>
    async def publish(self, req):           pub async fn create(&self, payload) -> Result<…>
        async with self.uow:                    let uow = self.uow_factory.build().await;
            ...                                 ...
            await self.uow.commit()             // commit() — future step
```

### Commit / Rollback in Rust

Rust's `Drop` trait is a destructor — it runs automatically when a value goes out of scope. This gives us auto-rollback: if the UoW is dropped without `commit()` being called, `Drop` rolls back the transaction. This covers panics, early `?` returns, and the "developer forgot" case.

However, `Drop` **cannot be async**. This is a fundamental Rust limitation — `drop()` is a synchronous function, and there's no way to `.await` inside it. This means:

- **`commit()` must be an explicit async call.** There is no way to auto-commit.
- **`rollback()` in `Drop` has to use `block_on()` or similar sync-over-async workaround**, OR the underlying transaction handle can be set to rollback-on-drop (sqlx's `Transaction` already does this — dropping an uncommitted `Transaction` rolls it back automatically, no async needed, because the DB connection itself handles it).

#### Can the compiler enforce that `commit()` is called?

Short answer: **not perfectly, but there are patterns that get close.**

**Option 1: `#[must_use]` on the return type (weak enforcement)**

```rust
#[must_use = "UoW must be committed — call .commit().await"]
pub struct UowResult<T> {
    pub value: T,
    uow: Box<dyn UnitOfWorkPort>,
}
```

The `#[must_use]` attribute makes the compiler emit a **warning** (not an error) if the caller ignores the return value. But it doesn't enforce that `commit()` is actually called on it — just that the value isn't silently dropped. Weak, but free.

**Option 2: Typestate pattern (strong enforcement, more ceremony)**

Encode the UoW's lifecycle into the type system. The UoW starts in a "pending" state and can only produce a result by transitioning to "committed":

```rust
// Factory returns this — you can write to it but can't extract a result
pub struct PendingUow { /* holds transaction */ }

// commit() consumes PendingUow and returns this — the only way to get your data out
pub struct CommittedUow<T> { pub value: T }

impl PendingUow {
    pub async fn commit<T>(self, value: T) -> Result<CommittedUow<T>, String> {
        // actually commits the transaction
        Ok(CommittedUow { value })
    }
    // Drop impl: rolls back if commit() was never called
}
```

The command method would return `Result<CommittedUow<ContactInquiry>, String>`. If someone tries to return a `ContactInquiry` without going through `commit()`, **they can't — it's inside `CommittedUow` and there's no other way to construct one.**

This is compile-time enforcement. The function literally cannot return a success value without committing. But it adds a wrapper type that callers must unwrap, and it forces a specific method signature shape.

**Option 3: Callback/closure pattern (structural enforcement)**

Instead of handing the UoW to the caller, the UoW runs a closure and commits after it succeeds:

```rust
impl dyn WriteUnitOfWorkFactory {
    pub async fn execute<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce(&dyn UnitOfWorkPort) -> Pin<Box<dyn Future<Output = Result<T, String>> + '_>>,
    {
        let uow = self.build().await;
        let result = f(&*uow).await?;
        uow.commit().await?;  // auto-commits on success
        Ok(result)
        // if f() returns Err, we skip commit → Drop rolls back
    }
}
```

Usage in the command method:
```rust
self.uow_factory.execute(|uow| Box::pin(async move {
    let created = uow.contact_inquiry_write_repo()
        .create_contact_inquiry(&prepared).await?;
    Ok(created)
})).await
```

The caller never sees `commit()` — it happens automatically when the closure returns `Ok`. This is the closest to Python's `async with self.uow:` pattern. The trade-off is the closure syntax with `Box::pin(async move { ... })` which is noisy in Rust.

#### Recommendation

Start with **explicit `commit()`** plus **`Drop`-based rollback** as the safety net. This is the most common pattern in Rust projects using sqlx transactions. It's simple, the auto-rollback catches mistakes, and you can add typestate or callback enforcement later if forgetting `commit()` becomes a real problem in practice.

The `Drop` rollback is the real guard. Even without compiler enforcement of `commit()`, the worst case is a silently rolled-back transaction — not silent data corruption. You'll notice quickly because the write won't persist.

---

## 6. HTTP Layer: Separate Request and Response DTOs

**Decision**: The API layer defines its own DTOs for both directions, separate from domain types.

```
Inbound:   JSON → ContactInquiryRequestBody (Deserialize) → From → ContactInquiryCreationPayload
Outbound:  ContactInquiry → From → ContactInquiryResponseBody (Serialize) → JSON
```

**Why**:
- Wire format can differ from domain (camelCase vs snake_case, field subsets)
- Domain entities never derive `Serialize`/`Deserialize` — those are infrastructure concerns
- The API layer owns the HTTP contract; the domain owns the business contract
- `From` impls at the boundary make the mapping explicit and testable

**POST handler returns 201**:
Axum's `Json(...)` defaults to 200. For creation, return a tuple `(StatusCode::CREATED, Json(...))` to get 201.

---

## 7. The Full Wiring (End to End)

```
HTTP POST /contact-inquiries
    │
    ▼
[api/handler.rs] create_contact_inquiry()
    │  Axum extracts: State(deps), Json(body)
    │  Builds ContactInquiryCommands from deps.write_uow_factory + deps.services
    │  Converts body DTO → ContactInquiryCreationPayload
    │  Calls commands.create(payload)
    │
    ▼
[application/commands.rs] ContactInquiryCommands::create()
    │  Calls spam_service.get_spam_likelihood(&payload.message) → u8
    │  Calls payload.prepare(likelihood) → ContactInquiryPrepared  (domain decides status)
    │  Calls uow_factory.build() → Box<dyn UnitOfWorkPort>
    │  Calls uow.contact_inquiry_write_repo().create_contact_inquiry(&prepared)
    │  Returns Ok(ContactInquiry)
    │
    ▼
[persistence/repo_adapter.rs] ContactInquiryWriteRepoSql::create_contact_inquiry()
    │  INSERT ... RETURNING * (binds fields from ContactInquiryPrepared)
    │  Maps returned row → ContactInquiryDbRow → ContactInquiry (via From)
    │  Returns the full entity with DB-assigned id, ident, timestamps
    │
    ▼
[api/handler.rs]
    │  Maps ContactInquiry → ContactInquiryResponseBody (via From)
    │  Returns (StatusCode::CREATED, Json(response_body))
```

---

## 8. struct_type Table (Status FK Pattern)

The database uses a `struct_type` table for status values. The domain sees statuses as strings ("unread", "spam", etc.) but the DB stores them as SMALLINT foreign keys.

The persistence layer handles the translation. When inserting, use a subquery:
```sql
(SELECT id FROM struct_type WHERE ident = $1 AND category = 'contact_inquiry_status')
```

When reading, JOIN struct_type to get the string back:
```sql
SELECT ci.*, st.ident as status FROM contact_inquiry ci
JOIN struct_type st ON ci.status_id = st.id
```

The domain never knows about integer IDs for statuses. That's a persistence detail.
