# Architecture Audit — Current State

> Reflects the codebase as of April 2026. For the original pre-cleanup audit, see [`archive/architecture_audit_v1.md`](archive/architecture_audit_v1.md).

## Overview

This repository is a **teaching/demo** project that demonstrates modular, hexagonal architecture in Rust using a library-management domain. The primary goal is to show clean separation between domain logic, application orchestration, persistence infrastructure, and transport layers — not to be a production-ready system.

## Workspace Structure

```
server/crates/
├── domain           # Pure domain: entities, typed IDs, status enums, port traits, errors
├── application      # CQRS commands & queries, orchestration, input types
├── persistence      # SQL adapters (sqlx), DB row mappings, unit-of-work impl
├── server_bootstrap # Dependency construction, config, re-exports for transport crates
├── http_server      # Axum REST transport (OpenAPI/Swagger)
├── graphql_server   # async-graphql transport
└── auth_core        # JWT auth adapter
```

## Architectural Strengths

### 1. Clean Crate Boundaries

- **`domain`** has zero infrastructure dependencies (only `anyhow`, `async-trait`, `chrono`, `thiserror`).
- **`application`** depends only on `domain` — it orchestrates via port traits.
- **`persistence`** implements the domain port traits with concrete SQL.
- **Transport crates** depend on `server_bootstrap` for wiring, not on `persistence` or `application` directly.

### 2. Transport Swappability

The strongest proof point: both `http_server` and `graphql_server` share the same `ServerDeps` from `server_bootstrap` and exercise identical domain/application paths. Neither transport knows about SQL or concrete persistence.

### 3. Typed IDs and Status Enums

All entity identifiers use newtype wrappers, and status fields use enums rather than raw strings:

| Domain | Types |
|--------|-------|
| Book | `BookId` |
| BookCopy | `BookCopyId`, `BookCopyStatus` (Active, Maintenance, Lost) |
| Member | `MemberId`, `MemberIdent`, `MemberStatus` (Active, Suspended) |
| Loan | `LoanId`, `LoanIdent` |

### 4. Explicit Domain Transitions

State transitions are owned by domain entities rather than being implicit in the application layer:

- `Member::suspend()` → `Result<MemberStatus, MemberError>`
- `Member::reactivate()` → `Result<MemberStatus, MemberError>`
- `BookCopy::send_to_maintenance()` → `Result<BookCopyStatus, BookCopyError>`
- `BookCopy::complete_maintenance()` → `Result<BookCopyStatus, BookCopyError>`
- `BookCopy::mark_lost()` → `Result<BookCopyStatus, BookCopyError>`
- `BookCopy::mark_found()` → `Result<BookCopyStatus, BookCopyError>`

Application commands call these transitions and pass the returned status to the persistence layer.

### 5. Create Returns Match DB Reality

All `INSERT ... RETURNING` statements return the full canonical row shape (including joins for status idents and derived fields). Persistence adapters map DB rows via `TryFrom` rather than manually reconstructing entities from input values.

### 6. API Contract Correctness

- `POST /books/{isbn}/copies` body contains only `{ barcode }` — no stale `isbn` or `author_name` in the request.
- `author_name` on `BookCopy` is derived from the parent `Book` via SQL join, never supplied at creation time.
- `BookCopy::is_borrowable()` clarifies that `Active` status means the copy is physically in circulation, not necessarily available to borrow (which also depends on active loan state).

## Unit-of-Work Pattern

Write operations use a transactional unit-of-work:

```
UnitOfWorkPort → exposes write repos (book, book_copy, member, loan)
                → commit()
```

Read operations use pooled connections directly, no transaction needed.

## Remaining Notes for Future Work

1. **Loan borrowability** — `BookCopyStatus::Active` does not mean "available to borrow" because loan state is the source of truth for whether a copy is checked out. The checkout command checks both `is_borrowable()` and active loan existence separately. A future iteration could add a `CheckedOut` status to `BookCopyStatus` if denormalizing loan state is desirable.

2. **Test coverage** — Domain transition tests exist to protect key invariants. Integration-level command tests can be expanded as the demo grows.

3. **Read model separation** — Queries currently return domain entities. A more evolved CQRS approach could use dedicated read DTOs to avoid loading full aggregates for read paths.
