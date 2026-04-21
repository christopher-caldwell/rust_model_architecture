# Hexagonal Architecture — Overview

## What This Is

This document describes the hexagonal (ports & adapters) architecture used in this Rust API server. The pattern enforces strict **dependency inversion**: inner layers define abstract interfaces (ports), and outer layers provide concrete implementations (adapters). Business logic never imports infrastructure. Infrastructure implements contracts defined by the business logic.

## Compiler-Enforced Boundaries via Cargo Workspace

This project uses a **Cargo Workspace** to physically enforce the architectural boundaries between layers. Each layer is its own crate with its own `Cargo.toml`. If a developer tries to import `sqlx` in the domain crate, the compiler rejects it because `sqlx` isn't in `domain/Cargo.toml`. If a developer tries to import persistence types in the API crate, the compiler rejects it because the `persistence` crate isn't listed in `api/Cargo.toml`.

This is not an honor system — **it is a hard barrier enforced by the Rust compiler.**

## The Five Crates

The workspace is organized into five crates under `crates/`, each representing an architectural layer. They are listed here from **innermost** (most protected) to **outermost** (most replaceable):

```
rust_server/
├── Cargo.toml                    # Workspace manifest
├── crates/
│   ├── domain/                   # 1. Innermost — pure business entities and contracts
│   │   ├── Cargo.toml            #    deps: chrono, async-trait
│   │   └── src/
│   ├── application/              # 2. Use-case orchestration layer
│   │   ├── Cargo.toml            #    deps: domain
│   │   └── src/
│   ├── persistence/              # 3. Infrastructure — database adapters
│   │   ├── Cargo.toml            #    deps: domain, application, sqlx
│   │   └── src/
│   ├── api/                      # 4. HTTP transport layer
│   │   ├── Cargo.toml            #    deps: domain, application, axum
│   │   └── src/
│   └── server/                   # 5. Composition root — wires everything together
│       ├── Cargo.toml            #    deps: domain, application, persistence, api
│       └── src/
├── docs/
└── Makefile
```

### Crate 1 — `domain`

The domain crate contains:

- **Entities** (`entity.rs`) — Plain data structs that represent core business objects. They have **zero** imports from any other crate. They may only use standard-library or foundational crates (`chrono`).
- **Repository Ports** (`repository_port.rs`) — `async_trait` trait definitions that declare what data-access operations the business logic needs, without specifying how they work. These are the primary "ports" in the hexagonal model.
- **Domain Exceptions** (`exceptions.rs`) — Custom error types representing domain-level violations (e.g., `UserIsBannedException`).

**Cargo.toml deps**: `async-trait`, `chrono` — nothing else. The compiler prevents importing any other crate.

### Crate 2 — `application`

The application crate contains:

- **Use-Case Services** — Separated into `queries.rs` (reads) and `commands.rs` (writes). Each service takes a `Box<dyn UnitOfWorkPort>` and orchestrates the business operation through it.
- **Unit-of-Work Port** (`uow_port.rs`) — A trait that bundles all repository ports together and provides transaction control (`start`, `commit`, `rollback`, `release`). This is the single integration point between application and persistence.
- **WriteUnitOfWorkFactory Port** (`uow_port.rs`) — A trait for creating scoped `UnitOfWork` instances (read-only or read-write).
- **Application Errors** (`errors.rs`) — Error types like `NotFound` and `Forbidden` that represent use-case-level outcomes.

**Cargo.toml deps**: `domain` — only. The compiler prevents any persistence or API imports.

### Crate 3 — `persistence`

The persistence crate contains:

- **Repository Adapters** — Concrete structs that implement the domain-layer repository port traits using `sqlx` or in-memory `HashMap`.
- **Mappers** (`mappers.rs`) — Types that convert between database row structs (`DbAuthor` with `sqlx::FromRow`) and domain entities (`Author`). The domain entity knows nothing about `sqlx`.
- **Unit-of-Work Adapter** (`database/uow.rs`) — `SqlUnitOfWork` implements `UnitOfWorkPort`. `SqlWriteUnitOfWorkFactory` implements `WriteUnitOfWorkFactory`.
- **Connection Pool Management** (`database/sql/pool.rs`) — `SqlPool` and `PoolLifecycle`.
- **Lookup Caches** (`struct_type/cache.rs`) — Boot-time hydrated caches like `StructTypeCache`.

**Cargo.toml deps**: `domain`, `application`, `sqlx`, `tokio`, `chrono`. The compiler prevents importing `api` or `axum`.

### Crate 4 — `api`

The API crate contains:

- **HTTP Handlers** — Axum handler functions that call application services and produce HTTP responses.
- **DTOs** — Response-shape structs (e.g., `AuthorDto`, `UserDto`) that derive `Serialize`.
- **Error Mapping** — Functions that downcast application errors to appropriate HTTP status codes.
- **Router** (`router.rs`) — Axum route registration.
- **Dependencies** (`dependencies.rs`) — `AppDeps` struct that holds `Arc<dyn WriteUnitOfWorkFactory>` (a trait object from the `application` crate).

**Cargo.toml deps**: `domain`, `application`, `axum`, `serde`, `serde_json`. **The `persistence` crate is NOT a dependency.** The compiler makes it physically impossible for any API handler to reference `sqlx`, `PgPool`, `SqlWriteUnitOfWorkFactory`, or any concrete database type.

### Crate 5 — `server` (Composition Root)

The server crate is the **only place** where all crates meet. It:

1. Creates `PoolLifecycle` (from `persistence`).
2. Hydrates `StructTypeCache` (from `persistence`).
3. Creates `SqlRepoFactory` and `SqlWriteUnitOfWorkFactory` (from `persistence`).
4. Erases the concrete type: `Arc::new(write_uow_factory) as Arc<dyn WriteUnitOfWorkFactory>`.
5. Passes the erased factory to `AppDeps` (from `api`).
6. Builds the Axum router and starts serving.

**Cargo.toml deps**: `domain`, `application`, `persistence`, `api`, `axum`, `tokio`, `dotenvy`.

## Dependency Flow Diagram

```
                        Cargo.toml deps (compile-time enforced)
                        ═══════════════════════════════════════

                    ┌──────────────────────────────────────────┐
                    │              server crate                │
                    │         (composition root)               │
                    │  deps: ALL crates (the only one)         │
                    └──┬──────────┬──────────┬──────────┬──────┘
                       │          │          │          │
                       ▼          │          │          ▼
                 ┌──────────┐    │          │    ┌──────────────┐
                 │api crate │    │          │    │ persistence  │
                 │          │    │          │    │    crate     │
                 │deps:     │    │          │    │deps:         │
                 │ domain   │    │          │    │ domain       │
                 │ applic.  │    │          │    │ application  │
                 │ axum     │    │          │    │ sqlx         │
                 └────┬─────┘    │          │    └──────┬───────┘
                      │          │          │           │
                      ▼          ▼          ▼           ▼
                 ┌────────────────────────────────────────┐
                 │          application crate             │
                 │   deps: domain (only)                  │
                 └───────────────┬────────────────────────┘
                                 │
                                 ▼
                 ┌────────────────────────────────────────┐
                 │            domain crate                │
                 │   deps: chrono, async-trait (only)     │
                 └────────────────────────────────────────┘
```

**Key insight: `api` and `persistence` are sibling adapters that cannot see each other.** They both point inward toward `application` and `domain`, but have no path to each other. Only the `server` crate (composition root) bridges them via trait-object type erasure.

## The Type-Erasure Boundary

The critical line in `server/main.rs`:

```rust
let deps = AppDeps {
    write_uow_factory: Arc::new(write_uow_factory) as Arc<dyn WriteUnitOfWorkFactory>,
};
```

This converts `Arc<SqlWriteUnitOfWorkFactory>` (a concrete persistence type) into `Arc<dyn WriteUnitOfWorkFactory>` (an abstract application trait). Once this cast happens, the concrete type is gone — the `api` crate can never reach back to `persistence`.

## Concurrency Model

- `Arc<dyn WriteUnitOfWorkFactory>` is shared across all request handlers via Axum's state system. No `RwLock` needed — the factory is fully initialized before serving begins.
- Each HTTP request creates its own `Box<dyn UnitOfWorkPort>` via the factory — no shared mutable state between requests.
- `StructTypeCache` uses `Arc<RwLock<HashMap>>` for concurrent reads with occasional lazy-fetch writes.
