# Unit of Work (UoW)

The Unit of Work is the transactional boundary for all write operations. It wraps a database transaction and provides access to repos that execute within that transaction.

---

## How It Works

```
pool.begin()  →  Transaction<'static, Postgres>
                        │
                        │  wrapped in Arc<Mutex<...>>
                        │  shared across repos
                        ▼
                  ┌─────────────┐
                  │ SqlUnitOfWork│
                  │  ├─ write_repo (uses tx)
                  │  └─ read_repo  (uses tx)
                  └─────────────┘
                        │
                        │  .commit()  →  unwraps Arc, commits tx
                        │  drop       →  automatic rollback
                        ▼
```

**Factory creates a UoW per command invocation:**

```rust
let uow = self.uow_factory.build().await;  // opens transaction
let result = uow.contact_inquiry_write_repo()
    .create_contact_inquiry(&prepared).await?;
uow.commit().await?;  // commits — if this line is never reached, tx rolls back
```

---

## Key Rules

### 1. Do non-DB work BEFORE building the UoW

The UoW holds a database connection for its entire lifetime. Anything that happens between `build()` and `commit()`/drop holds that connection open. External calls (LLM, HTTP, etc.) must happen before `build()`.

```rust
// Correct — connection held only during DB ops
let spam_rating = self.spam_service.get_spam_likelihood(&msg).await?;
let prepared = payload.prepare(ident, spam_rating);
let uow = self.uow_factory.build().await;       // connection acquired HERE
uow.write_repo().create(&prepared).await?;
uow.commit().await?;                             // connection released HERE

// Wrong — connection held during LLM call
let uow = self.uow_factory.build().await;       // connection acquired HERE
let spam_rating = self.spam_service.get_spam_likelihood(&msg).await?;  // 500ms-2s idle connection
uow.write_repo().create(&prepared).await?;
uow.commit().await?;
```

With `max_connections(5)`, the wrong ordering means 5 concurrent requests exhaust the pool while waiting on non-DB work.

### 2. Commit is explicit, rollback is automatic

- `commit(self: Box<Self>)` consumes the UoW and commits the transaction.
- If the UoW is dropped without `commit()` (early `?` return, error, panic), the transaction rolls back automatically. This is SQLx's built-in behavior — dropping an uncommitted `Transaction` sends a ROLLBACK.
- The worst failure mode is a silently rolled-back write, not silent data corruption.

### 3. Repos inside the UoW share one transaction

All repos in a UoW execute against the same `Transaction` via `Arc<Mutex<Transaction>>`. This means:
- Writes are visible to reads within the same UoW (read-your-writes consistency)
- All operations commit or rollback atomically
- The mutex ensures only one query runs against the transaction at a time (SQLx requirement — `Transaction` needs `&mut self`)

---

## Adding a New Entity's Repos to the UoW

1. **Create the Tx repo variant** in the persistence crate. Copy the pool-based repo and change:
   - Struct field: `pub tx: Arc<Mutex<Transaction<'static, Postgres>>>` instead of `pub pool: PgPool`
   - Each method: `let mut tx = self.tx.lock().await;` then execute against `&mut **tx`

2. **Add to `SqlUnitOfWork`** — add a field for the new Tx repo, wire it in `UnitOfWorkPort` impl.

3. **Add to the port trait** — add an accessor method to `UnitOfWorkPort` in `application/src/uow_port.rs`.

4. **Wire in `build()`** — pass `tx.clone()` to the new repo in `SqlWriteUnitOfWorkFactory::build()`.

5. **Make the module `pub(crate)`** in `mod.rs` so `uow.rs` can access the Tx type.

---

## Architecture Notes

- **Pool-based repos still exist** for the read path (queries outside a write transaction). They are used via `ReadRepos` and the `ro_pool`. The Tx variants are only used inside the UoW.
- **The domain layer knows nothing about transactions.** The port traits (`ContactInquiryWriteRepoPort`, etc.) have no transaction-related methods. Transaction scoping is an infrastructure concern handled entirely in the persistence and application layers.
- See [decisions.md](decisions.md) sections 5 (Commands pattern) and the commit/rollback discussion for design rationale.
