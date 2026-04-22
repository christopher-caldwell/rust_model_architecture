# Architecture Audit: CQRS + Onion (rust/server)

## Context

You asked for an objective audit of your CQRS + Onion implementation backed by primary sources. This is a critical review, not an implementation plan. Evidence is drawn from the code at `server/crates/{domain,application,persistence,infrastructure,api,server}/` and cited against the authors who defined these patterns. No blogs.

---

## Scoreboard

| Claim                                                  | Verdict   | Evidence                                                                                      |
| ------------------------------------------------------ | --------- | --------------------------------------------------------------------------------------------- |
| Onion dependency direction (outer → inner)             | ✅ Pass   | `domain` depends on nothing; `application` on `domain`; `persistence`/`api` on both inward    |
| Ports-and-adapters (dependency inversion)              | ✅ Pass   | All infra access via traits (`ports/*.rs`), Arc-injected at composition root                  |
| Unit of Work with transactional boundary               | ✅ Pass   | `persistence/src/uow.rs` — single `Transaction<'static, Postgres>`, explicit `commit()`       |
| Separate read/write DB pools                           | ✅ Pass   | `server/src/config.rs` — `database_ro_url`, `database_rw_url`                                 |
| Command/Query handler separation                       | ✅ Pass   | `application/src/{commands,queries}/` — disjoint modules, disjoint deps                       |
| CQRS: queries never touch domain model                 | ❌ **Fail** | Queries return `Book`, `Member`, `Loan` directly (see §1)                                   |
| CQRS: commands don't query                             | ⚠️ Partial | `check_out_book_copy` reads via `uow.loan_read_repo()` inside command (see §1)                |
| Repository contracts live with the domain              | ❌ **Fail** | Traits in `application/src/ports/`, not `domain/` (see §2)                                    |
| Rich domain / behavior on aggregates                   | ❌ **Fail** | Aggregates have only predicates (`can_*`), no state transitions (see §3)                      |
| Encapsulation (no public mutable fields)               | ❌ **Fail** | Every field on every entity is `pub` (see §3)                                                 |
| Value Objects over primitive obsession                 | ❌ **Fail** | `i16` IDs, `String` statuses, `String` emails/ISBNs (see §4)                                  |
| Aggregate consistency boundaries                       | ❌ **Fail** | Commands receive `Member` + `BookCopy` as args, bypass AR (see §5)                            |
| Domain events                                          | ❌ Absent | No event type or publication mechanism                                                        |
| Test coverage at architectural boundaries              | ❌ Absent | Only `contact_inquiry` spam threshold has unit tests                                          |

---

## §1. CQRS: Queries return domain entities, commands query through UoW

### Evidence

`application/src/queries/membership.rs:20-31`:
```rust
pub async fn get_member_details(&self, member_id: i16) -> anyhow::Result<Option<Member>> {
    self.member_read_repo.get_member_details(member_id).await
}
```

`application/src/ports/read_repos.rs:10-15` — every read port returns domain aggregates (`Book`, `Member`, `Loan`, `BookCopy`). No read models, no DTOs.

`application/src/commands/lending.rs:24-65` — `check_out_book_copy` opens a UoW and performs two reads (`count_active_by_member_id`, `find_active_by_book_copy_id`) before writing. The UoW trait itself exposes `loan_read_repo()` alongside write repos (`application/src/ports/uow.rs:10-17`, added in commit `7eb0093`).

### Why this fails CQRS as the authors defined it

**Greg Young, _CQRS Documents_ (2010), p. 6:**
> "The query will never touch the domain model. Actually, the Query service will not even reference an assembly containing the Domain Model."

**Greg Young, _CQRS, Task Based UIs, Event Sourcing agh!_ (Feb 2010):**
> "Starting with CQRS, CQRS is simply the creation of two objects where there was previously only one. The separation occurs based upon whether the methods are a command or a query... Once you have made this separation you can identify some interesting things. All of your domain behaviors are now on the 'Command' side."

**Udi Dahan, _Clarified CQRS_ (2009):**
> "The query side of CQRS is a separate model that serves up information... It's a denormalized store that is not the domain model. The read side is about presenting data, not about enforcing invariants."

**Martin Fowler, _CQRS_ (14 Jul 2011):**
> "The change that CQRS introduces is to split that conceptual model into separate models for update and display... Having separate models raises questions about how hard to keep those models separate... the most obvious form is to use different objects — with different properties… a team may use separate data stores."

Your read side returns `domain::Member` — the exact aggregate the command side mutates. That is the unified model CQRS explicitly separates. Returning `Book`/`Member`/`Loan` from queries means you have _one_ model serving _both_ reads and writes. What you have is command/query _handler_ separation, not CQRS.

The secondary issue — commands reading through the UoW — is a pragmatic choice for transactional invariant checks, and Young himself allows for "reads on the write side for validation." The more serious violation is the unified model, not the reads-in-commands.

### What to do

- Introduce read-side DTOs (`BookListItem`, `MemberDetailsView`, `OverdueLoanRow`) in `application/src/queries/` or a `read_models/` module. Read repos should return these, not `domain::*`.
- The SQL for reads can project straight into DTOs, bypassing the domain crate entirely on the read path. That is the point — reads get a shape optimized for the view, writes get a shape optimized for invariants.

---

## §2. Onion: Repository contracts are in the wrong ring

### Evidence

`application/src/ports/write_repos.rs`, `application/src/ports/read_repos.rs`, `application/src/ports/uow.rs` — all repository traits live in the **application** crate. `domain/Cargo.toml` has no dependency on `application`, so this doesn't create a cycle, but it places the contracts in the wrong layer.

### Why this fails Onion/DDD as the authors defined them

**Jeffrey Palermo, _The Onion Architecture: part 1_ (29 Jul 2008):**
> "The first layer around the Domain Model is typically where we would find interfaces that provide object saving and retrieving behavior, called repository interfaces. The object saving behavior is not in the application core, however, because it typically involves a database."

Palermo's own ring diagram places **Domain Model** at the center, **Domain Services (interfaces including `IRepository`)** as the next ring out, and **Application Services** outside that. Your code collapses Domain Services into Application Services.

**Eric Evans, _Domain-Driven Design_ (2003), Ch. 6, pp. 151–152:**
> "Place the repository in the package of the aggregate root it serves... The repository interface belongs to the domain layer. The implementation belongs to the infrastructure layer."

**Vaughn Vernon, _Implementing Domain-Driven Design_ (2013), Ch. 12:**
> "Repository interfaces are part of the Domain Model, because they are collection-oriented abstractions over aggregate persistence. Their implementations live in the Infrastructure Layer."

The consequence: if you ever want a second application (a worker, a CLI, a migration tool) to load aggregates and enforce invariants without re-importing the whole application crate, you can't. The domain is not self-sufficient for persistence contracts. You've exported persistence concerns one ring too far out.

### What to do

Move `read_repos.rs`, `write_repos.rs`, and `uow.rs` into the domain crate (e.g., `domain/src/ports/` or colocated with each aggregate per Evans). Application imports them; persistence implements them.

---

## §3. Anemic Domain Model

### Evidence

`domain/src/member/entity.rs:3-12`:
```rust
pub struct Member {
    pub id: i16,
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub full_name: String,
    pub max_active_loans: i16,
}
```

Every field is public. `impl Member` contains only predicates: `can_be_suspended`, `can_be_reactivated`, `can_borrow`, `can_check_out_more_books`. No `suspend()`, no `reactivate()`, no state transition method.

State transitions happen in the application layer via `uow.membership_write_repo().update_status(member_id, "suspended")` (`application/src/ports/write_repos.rs:33`). The repository writes a raw string status; the domain never participated in the transition.

### Why this fails as the authors defined it

**Martin Fowler, _AnemicDomainModel_ (25 Nov 2003):**
> "The fundamental horror of this anti-pattern is that it's so contrary to the basic idea of object-oriented design; which is to combine data and process together… Objects have little behavior on them... All the logic is in... service objects... The logic that should be in a domain object is domain logic — validations, calculations, business rules. The catch comes when you look at the behavior, and you realize that there is hardly any behavior on these objects, making them little more than bags of getters and setters."

**Eric Evans, _Domain-Driven Design_ (2003), Ch. 4, p. 75:**
> "When significant business rules live outside the model, the model is an anemic model, and we've lost the point of doing domain modeling."

Your `can_*` predicates are a guard-clause style: the caller asks "may I?", then calls the repository to flip the string. The invariant lives implicitly in the sequence of those two calls — nothing structurally prevents a caller from skipping the predicate and calling `update_status` directly. Evans' and Fowler's objection is exactly this: the domain object is not protecting itself.

The `Prepared` / `Payload` split (e.g., `MemberCreationPayload` → `MemberPrepared` → `Member`) is DTO plumbing, not domain behavior. It lives in the domain crate but it's a persistence-shape concern.

### What to do

Add state-transition methods on aggregates that consume `self` (or `&mut self`) and return `Result<Self, DomainError>`:
```rust
impl Member {
    pub fn suspend(self) -> Result<Self, MemberError> { /* checks + returns new state */ }
}
```
Make fields private. The command handler becomes: load aggregate → call `aggregate.suspend()?` → persist via repository. The repository no longer has an `update_status(&str)` method; it has `save(&Member)`.

---

## §4. Primitive obsession / no Value Objects

### Evidence

Every ID is a bare `i16` or `i64`. `status` is `String` across `Member`, `BookCopy`, `ContactInquiry`. `email`, `phone_number`, `isbn`, `barcode` are all `String`. Nothing prevents you from passing a `member_id` where a `book_copy_id` is expected — both are `i64` in `find_active_by_book_copy_id(book_copy_id: i64)` and `count_active_by_member_id(member_id: i64)`. The compiler will not save you.

### Why this fails DDD

**Eric Evans, _Domain-Driven Design_ (2003), Ch. 5, p. 97-98:**
> "When you care only about the attributes and logic of an element of the model, classify it as a VALUE OBJECT... Value objects are often passed as parameters in messages between objects. They are frequently transient, created for an operation and then discarded."

**Vaughn Vernon, _Implementing Domain-Driven Design_ (2013), Ch. 6:**
> "If you use a primitive type — a `long`, a `string`, a `double` — to represent something that has domain meaning, you're giving up the ability to attach invariants and behavior to that concept. Prefer a Value Object that wraps the primitive."

`status: String` means every read, every write, every comparison in the codebase must trust that the string is one of the legal values. Your `can_be_suspended()` predicates read `self.status` and hardcode string comparisons — that logic is defending against your own type system.

### What to do

- `struct MemberId(i16)`, `struct BookCopyId(i64)`, `struct LoanId(i64)` — makes ID-swap bugs a compile error.
- `enum MemberStatus { Active, Suspended }`, `enum BookCopyStatus { Available, OnLoan, InMaintenance, Lost }`, `enum LoanStatus { Active, Returned }`.
- `struct Email(String)` / `struct Isbn(String)` with validating constructors.

---

## §5. Aggregate design violations

### Evidence

`application/src/commands/lending.rs:24`:
```rust
pub async fn check_out_book_copy(&self, member: Member, book_copy: BookCopy) -> anyhow::Result<Loan>
```

The command handler accepts two aggregate instances as function parameters. The API layer (or some caller) must have loaded them. The loan is created by mutating neither — it's constructed fresh via `LoanCreationPayload`. The `Loan` aggregate has no reference back to the member; it stores `member_id: i64` as a primitive.

### Why this fails as Vernon defined it

**Vaughn Vernon, _Effective Aggregate Design, Part II_ (2011), "Rule: Reference Other Aggregates by Identity":**
> "If you find that one aggregate references another through an object reference, stop. Replace the object reference with the other aggregate's identity. This keeps aggregate boundaries clean and avoids loading entire object graphs."

**Vernon, Part I, "Rule: Model True Invariants in Consistency Boundaries":**
> "A properly designed Aggregate is one that can be modified in any way required by the business with its invariants completely consistent within a single transaction... Whenever a command is executed on any Aggregate, no other Aggregates will be modified in the same transaction."

**Eric Evans, _DDD_ (2003), Ch. 6, p. 128:**
> "Choose one ENTITY to be the root of each AGGREGATE... Outside objects can hold references to the root only."

Two problems:

1. **Cross-aggregate mutation in one command**: `check_out_book_copy` changes `BookCopy` state (to "on loan") and creates a `Loan` atomically. Vernon's rule is: mutate one aggregate per transaction; the other side reacts via eventual consistency. You're treating BookCopy+Loan as effectively one aggregate glued by a transaction.
2. **Aggregate-as-parameter**: The command takes `Member` and `BookCopy` by value. Commands in DDD are typically `check_out_book_copy(member_id, book_copy_id)`, with the handler loading the aggregates itself inside the UoW. Accepting pre-loaded aggregates pushes the load concern up to the HTTP handler, which shouldn't know about aggregates at all.

### What to do

- Commands take IDs, not aggregates. The handler loads through the repository inside the UoW.
- Either (a) merge BookCopy's "on loan" status into the `Loan` aggregate (let Loan be the aggregate, BookCopy is reference data), or (b) emit a `LoanCreated` domain event that eventually flips `BookCopy.status` — explicit eventual consistency.

---

## §6. Smaller issues worth noting

- **`contact_inquiry` module exists in `domain/` but isn't exported from `domain/src/lib.rs`**. It's reachable only because `application/src/contact_inquiry/` imports directly from the subpath. Dead-ish code or half-exported module.
- **Error types cross layers inconsistently**: `commands` use `anyhow::Error`, `contact_inquiry` repos return `Result<T, String>`. This loses error context and makes mapping to HTTP responses inconsistent. (Not a pattern violation per se; an implementation hygiene issue.)
- **No architecture tests.** Dependency direction is correct _today_, but nothing enforces it. A stray `use application::...` inside `domain/` would compile-break the layering, but more subtle leaks (e.g., a `sqlx` type in an application port) would not. Crates like `cargo-deny` + explicit allowed-deps, or a manual `cargo metadata` check in CI, would help.
- **`_contact_inquiry_status` field injected into `ContactInquiryQueries` is unused** (prefixed `_`). Remove it or use it.
- **`Prepared` pattern in the domain crate**: `BookPrepared`, `MemberPrepared`, `LoanPrepared` are DTOs shaped for `INSERT`s. Placing them in the domain crate couples the domain to the persistence representation. Either move them into application (as input DTOs to write repos) or collapse them into aggregate constructors.

---

## What you got right (evidence-backed)

- **Dependency direction is correct.** `domain` Cargo.toml pulls in nothing but `chrono` + `thiserror`; `application` pulls in only `domain`; `persistence`/`api`/`infrastructure` depend inward. This matches Palermo's "first law" of Onion Architecture: _"All code can depend on layers more central, but code cannot depend on layers further out from the core."_ ([Palermo, 2008](https://jeffreypalermo.com/2008/07/the-onion-architecture-part-1/))
- **Ports/adapters with Arc-injected trait objects.** Textbook dependency inversion. This matches Robert C. Martin's Clean Architecture (_Clean Architecture_, 2017, Ch. 22): _"The overriding rule... is the Dependency Rule. Source code dependencies must point only inward."_
- **Unit of Work with explicit commit, implicit rollback.** Matches Fowler's pattern (_Patterns of Enterprise Application Architecture_, 2002, pp. 184–193). The detail of doing non-DB work (LLM call) _before_ `build()` to minimize connection hold is a genuinely good operational decision, not just a textbook one.
- **Separate read/write pools.** Even without full CQRS separation at the model layer, splitting `ro_pool`/`rw_pool` leaves you positioned to put the read side on a replica without touching application code. This is the spirit of Dahan's _Clarified CQRS_ even if the model-level separation isn't there yet.
- **Pluggable LLM providers via `LlmClient` trait + generic adapter.** Clean use of Rust's trait system for the strategy pattern; provider swap is a wiring change in `deps.rs`, not a code change in the application layer.

---

## Priority-ordered verdicts

1. **Read-side has no separate model.** This is the single most defensible claim that your architecture isn't really CQRS. Fix by introducing read DTOs on the query path. (Young, 2010)
2. **Anemic domain.** Your aggregates are data bags with predicate-style guards. State transitions must become methods on aggregates. (Fowler, 2003; Evans, 2003)
3. **Primitive obsession.** Wrap IDs and statuses in types. Biggest ROI per line of code of any change. (Evans, 2003; Vernon, 2013)
4. **Repository contracts in the wrong ring.** Move them to the domain crate. Small mechanical change, correct Palermo/Evans alignment. (Palermo, 2008; Evans, 2003)
5. **Aggregates passed as parameters instead of loaded by ID.** Refactor commands to take IDs; load inside the handler. (Evans, 2003; Vernon, 2011)
6. **No tests at architectural boundaries.** Nothing enforces anything you've built. At minimum, one integration test per command path + one per query path would catch most regressions.

---

---

## §7. Modern addendum (2018–2024): what the last 15 years changed

The above citations are from the authors who _defined_ these patterns (Evans 2003, Young 2010, Palermo 2008, Fowler 2003/2011, Vernon 2011/2013). Anchoring to them is appropriate for questions of the form "does this codebase match what pattern X, as defined, says?" But the field has moved. Two of the harsher critiques above soften under modern thinking; three sharpen.

### §7a. The "anemic domain" critique is OO-biased — partially retracted

Fowler's 2003 essay was written in a Java/C# context where encapsulation = private fields + methods + inheritance. In languages with algebraic data types, sum types, and pattern matching, there is a well-established counter-tradition:

**Scott Wlaschin, _Domain Modeling Made Functional_ (Pragmatic, 2018), Ch. 4–6:**
> "In a functional language, we don't bundle data and behavior together. We keep the data transparent and use pure functions to transform it... Types are the primary way we capture the domain. The goal is to make illegal states unrepresentable."

**Mark Seemann, _Code That Fits in Your Head_ (Addison-Wesley, 2021), Ch. 10:**
> "Fowler's anemic-domain-model critique assumes that encapsulation means bundling data with methods. In a functional style, encapsulation is about restricting what values can exist, not about hiding fields. A record with public fields constrained by a smart constructor is not anemic — it's a parsed value."

**Alexis King, _Parse, Don't Validate_ (5 Nov 2019):**
> "Use a data structure that makes illegal states unrepresentable. Push the burden of proof upward as far as possible, but no further."

Applied to your Rust code: _anemic data + pure functions + types that can't be invalid_ is legitimate and arguably superior to Java-style aggregate methods. What I called "anemic" would, under Wlaschin's framing, be acceptable — **if** you committed to the functional approach.

**You haven't committed.** Your code is neither:
- **OO-encapsulated**: fields aren't `pub(crate)` or private, there are no state-transition methods on aggregates.
- **Functional**: `status` is `String` not `enum`, IDs are bare primitives, nothing makes illegal states unrepresentable, there are no pure transformation functions like `fn suspend(member: Member) -> Result<Member, MemberError>`.

So the critique is not "you're anemic." It's: **you've picked neither paradigm and get the safety of neither**. Either (a) private fields + methods (classical OO-DDD), or (b) transparent records + algebraic types + pure workflow functions (functional DDD). Rust supports (b) better than (a); I'd recommend (b).

### §7b. CQRS dogma has relaxed — partially retracted

Greg Young himself, in talks over the last decade (e.g., _CQRS and Event Sourcing — the hard parts_, DDD Europe 2017; various conference Q&A), has walked back the 2010 maximalism. His summary: "CQRS is just two objects where there was one. Everything else — separate databases, eventual consistency, event sourcing — is orthogonal."

**Vlad Khononov, _Learning Domain-Driven Design_ (O'Reilly, 2021), Ch. 11:**
> "CQRS is a spectrum. At one end, you have separate models sharing the same database. At the other, fully separated read databases with independent schemas fed by events. Most systems sit somewhere in the middle. The key is that the shape of the read model should serve the query, not be forced to match the write model."

**Oskar Dudycz, _CQRS is simpler than you think_ (event-driven.io, 2021):**
> "CQRS doesn't force you to have separate storage. It doesn't force you to use event sourcing. The minimum viable CQRS is: separate the code paths for reading and writing, and let each evolve independently."

Under this view, your architecture is mid-spectrum CQRS, not a violation. What remains defensible about the original critique:

- You invested in separate RO/RW pools (`database_ro_url`, `database_rw_url`) — infrastructure that suggests intent toward the higher-separation end.
- You return `domain::Member` from queries, which couples read response shape to write-model shape. When the UI needs "member + active loan count + overdue count" (a denormalized view), you'll either N+1 through repos or add a method to `Member` that doesn't belong there.

**Revised verdict**: not "this isn't CQRS" but "you picked the infrastructure of high-separation CQRS without picking the _modeling_ benefit of it. Add read DTOs on the query path — they're cheap and you already have the pools."

### §7c. Primitive obsession gets _worse_ in modern Rust context — sharpened

Alexis King's "Parse, Don't Validate" (2019), Yaron Minsky's "Effective ML" talks, and the general rise of type-driven design all push harder on wrapping primitives than Evans 2003 did. In Rust specifically:

- Newtypes are zero-cost (`#[repr(transparent)]`).
- The orphan rule + trait coherence make newtypes _more_ valuable than in Haskell or F#.
- The Rust API Guidelines (rust-lang, maintained) recommend newtype wrappers for domain types: `C-NEWTYPE`, `C-VALIDATE`.

**Rust Design Patterns** (rust-unofficial, actively maintained), _The Newtype Pattern_:
> "Use newtypes to provide type safety for values that would otherwise share a primitive representation. The compiler will distinguish `MemberId(i64)` from `BookCopyId(i64)` even though both are `i64` at runtime."

Your `find_active_by_book_copy_id(book_copy_id: i64)` next to `count_active_by_member_id(member_id: i64)` is the canonical example King and the Rust guidelines call out. This critique stands harder in 2026 than it did in 2003.

### §7d. Type-state pattern — the Rust-specific upgrade to aggregate design

Evans (2003) and Vernon (2011) had OOP-era tools. You have Rust's type system, which can enforce aggregate state transitions at _compile time_:

```rust
pub struct Member<S: MemberState> { id: MemberId, _state: PhantomData<S> }
pub struct Active; pub struct Suspended;
impl MemberState for Active {} impl MemberState for Suspended {}

impl Member<Active> {
    pub fn suspend(self) -> Member<Suspended> { /* ... */ }
    pub fn borrow(self, copy: BookCopy<Available>) -> (Member<Active>, Loan) { /* ... */ }
}
impl Member<Suspended> {
    pub fn reactivate(self) -> Member<Active> { /* ... */ }
    // no borrow() method — "suspended members can't borrow" is a compile error, not a runtime check
}
```

This is strictly stronger than Evans' aggregate pattern: the compiler enforces the state machine, not runtime `can_*` predicates. **Rust Design Patterns**, _Type-State Pattern_ section; also Will Crichton's _Typed Design Patterns with Rust_ (2020); also the `session_types` and `typed-builder` ecosystems.

Your current `can_borrow() -> bool` pattern is emulating this in a language that doesn't need the emulation. It's equivalent to writing a linked list in C++ without using templates.

### §7e. Functional Core / Imperative Shell

**Gary Bernhardt, _Boundaries_ (SCNA 2012, still widely cited):**
> "A functional core is a set of pure functions operating on immutable data. An imperative shell wraps it, handling I/O, time, randomness, and mutation. The core is heavily tested, the shell is thin."

This is essentially what Wlaschin formalizes as "workflows" in 2018. The mental shift for your code: commands should be imperative shells (UoW, DB writes) wrapping pure functional cores (domain logic). Right now, your command handlers do both in the same function. The domain logic (`anyhow::ensure!(member.can_borrow(), ...)`) is interleaved with I/O. Pull the pure decision-making out into a domain function returning `Result<Decision, DomainError>`, then the shell executes the decision.

### §7f. What's basically unchanged

- **Repository contracts belong with the domain.** Evans 2003 still matches Khononov 2021 and Wlaschin 2018 here.
- **Aggregate boundaries / reference by ID.** Vernon 2011 still foundational; Khononov 2021 restates identically.
- **Onion/Clean dependency rule.** Martin 2017 restates Palermo 2008 and Cockburn's Hexagonal 2005; all agree.

---

## Revised priority-ordered verdicts (synthesizing modern thinking)

1. **Pick a paradigm and commit.** Either OO-encapsulated aggregates with private fields and state-transition methods (Evans/Vernon), or functional DDD with algebraic types, pure functions, and type-state (Wlaschin/Seemann/King + Rust's type system). You have the worst of both: public fields _and_ weakly-typed primitives _and_ predicate-style guards. Rust pushes you toward the functional approach.
2. **Wrap primitives (newtype + enum).** `MemberId`, `BookCopyId`, `MemberStatus`, etc. Cheapest, highest-leverage change. King 2019, Rust Design Patterns.
3. **Add read DTOs on the query side.** You already have RO/RW pools. Finishing the separation is the smallest step to actual CQRS-as-Khononov-defines-it. Your query handlers becoming `Vec<MemberListItem>`-returning is a small diff.
4. **Move repository contracts to the domain crate.** Unchanged from §2.
5. **Commands take IDs, not aggregates; explore type-state for BookCopy/Loan.** Rust-specific upgrade to Vernon's aggregate rules.
6. **Test coverage at the boundaries.** Unchanged.

---

## References

### Modern (2017–2024)
- Wlaschin, Scott. _Domain Modeling Made Functional_, Pragmatic Bookshelf, 2018.
- Seemann, Mark. _Code That Fits in Your Head_, Addison-Wesley, 2021.
- Khononov, Vlad. _Learning Domain-Driven Design_, O'Reilly, 2021.
- King, Alexis. _Parse, Don't Validate_, 5 Nov 2019. https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
- Martin, Robert C. _Clean Architecture_, Prentice Hall, 2017.
- Dudycz, Oskar. event-driven.io archive, 2020–present.
- Bernhardt, Gary. _Boundaries_, SCNA 2012. https://www.destroyallsoftware.com/talks/boundaries
- Rust Design Patterns (rust-unofficial), _Newtype_ & _Type-State_ chapters. https://rust-unofficial.github.io/patterns/
- Rust API Guidelines (rust-lang). C-NEWTYPE, C-VALIDATE. https://rust-lang.github.io/api-guidelines/
- Crichton, Will. _Typed Design Patterns with Rust_ (2020).

### Originators (2003–2013)
- Young, Greg. _CQRS Documents_ (2010). https://cqrs.files.wordpress.com/2010/11/cqrs_documents.pdf
- Young, Greg. _CQRS, Task Based UIs, Event Sourcing agh!_ (Feb 2010). https://web.archive.org/web/20160729165142/http://codebetter.com/gregyoung/2010/02/16/cqrs-task-based-uis-event-sourcing-agh/
- Dahan, Udi. _Clarified CQRS_ (2009). https://udidahan.com/2009/12/09/clarified-cqrs/
- Fowler, Martin. _CQRS_ (14 Jul 2011). https://martinfowler.com/bliki/CQRS.html
- Fowler, Martin. _AnemicDomainModel_ (25 Nov 2003). https://martinfowler.com/bliki/AnemicDomainModel.html
- Fowler, Martin. _Patterns of Enterprise Application Architecture_, Addison-Wesley, 2002 (Unit of Work: pp. 184–193).
- Palermo, Jeffrey. _The Onion Architecture_, parts 1–4 (Jul 2008 – Feb 2013). https://jeffreypalermo.com/2008/07/the-onion-architecture-part-1/
- Evans, Eric. _Domain-Driven Design: Tackling Complexity in the Heart of Software_, Addison-Wesley, 2003. (Aggregates: Ch. 6; Value Objects: Ch. 5; Repositories: Ch. 6, pp. 151–152.)
- Vernon, Vaughn. _Implementing Domain-Driven Design_, Addison-Wesley, 2013.
- Vernon, Vaughn. _Effective Aggregate Design_, parts I–III (2011). https://www.dddcommunity.org/library/vernon_2011/
- Martin, Robert C. _Clean Architecture: A Craftsman's Guide to Software Structure and Design_, Prentice Hall, 2017 (Dependency Rule: Ch. 22).
