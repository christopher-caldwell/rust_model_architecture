# Hexagonal Architecture — Implementation Guide (From Scratch)

This document describes the exact **order of operations** for implementing this hexagonal architecture pattern in a new Rust project using a **Cargo Workspace** to enforce the layer boundaries at compile time.

---

## Phase 1: Workspace Scaffold

### Step 1 — Create the workspace root

```bash
mkdir my_server && cd my_server
mkdir -p crates/{domain,application,persistence,api,server}/src
```

### Step 2 — Create the workspace `Cargo.toml`

```toml
[workspace]
members = [
    "crates/domain",
    "crates/application",
    "crates/persistence",
    "crates/api",
    "crates/server",
]
resolver = "2"
```

This file has **no `[package]` section** — it is purely a workspace manifest. Individual crates have their own `Cargo.toml`.

### Step 3 — Create each crate's `Cargo.toml`

**`crates/domain/Cargo.toml`** — Zero internal dependencies:
```toml
[package]
name = "domain"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
```

**`crates/application/Cargo.toml`** — Depends on `domain` only:
```toml
[package]
name = "application"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1"
domain = { path = "../domain" }
```

**`crates/persistence/Cargo.toml`** — Depends on `domain` + `application`:
```toml
[package]
name = "persistence"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1"
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "chrono", "uuid"] }
tokio = { version = "1", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
domain = { path = "../domain" }
application = { path = "../application" }
```

**`crates/api/Cargo.toml`** — Depends on `domain` + `application`, **NOT** `persistence`:
```toml
[package]
name = "api"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
domain = { path = "../domain" }
application = { path = "../application" }
```

**`crates/server/Cargo.toml`** — Depends on everything:
```toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
dotenvy = "0.15"
domain = { path = "../domain" }
application = { path = "../application" }
persistence = { path = "../persistence" }
api = { path = "../api" }
```

> **The enforcement happens here.** Notice that `api/Cargo.toml` lists `domain` and `application` but **not** `persistence`. Any developer who writes `use persistence::...` in any file inside `crates/api/` will get a compiler error. The barrier is physical, not conventional.

---

## Phase 2: Domain Crate (Build This First)

**Why start here?** The domain crate has zero dependencies on anything else. It compiles on its own. Every other crate depends on it, so it must exist first.

### Step 4 — Create `crates/domain/src/lib.rs`

```rust
pub mod author;
pub mod user;
```

### Step 5 — Define your entity

Create `crates/domain/src/author/entity.rs`:

```rust
use chrono::{DateTime, Utc};

/// Domain entity — zero imports from other crates.
#[derive(Debug, Clone)]
pub struct Author {
    pub author_id: i32,
    pub author_uuid: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub first_name: String,
    pub last_name: String,
}
```

Key rules:
- **No `Serialize`/`Deserialize`** — the domain doesn't know about JSON.
- **No `sqlx::FromRow`** — the domain doesn't know about databases.
- **No framework types** — only standard library and foundational crates.
- **`status` is a `String`**, not an integer FK — the domain speaks in business language.

### Step 6 — Define the repository port

Create `crates/domain/src/author/repository_port.rs`:

```rust
use async_trait::async_trait;
use super::entity::Author;

/// Read-side repository contract.
#[async_trait]
pub trait AuthorReadRepoPort: Send + Sync {
    async fn get_by_uuid(
        &self,
        author_uuid: &str,
    ) -> Result<Option<Author>, Box<dyn std::error::Error + Send + Sync>>;

    async fn list_all(
        &self,
    ) -> Result<Vec<Author>, Box<dyn std::error::Error + Send + Sync>>;
}

/// Write-side repository contract.
pub trait AuthorWriteRepoPort: Send + Sync {}
```

Key rules:
- Uses `async_trait` because Rust requires it for async methods in traits.
- Returns `Box<dyn std::error::Error + Send + Sync>` — the domain doesn't know what error types infrastructure might produce.
- Methods take `&self`, not database connections or pools.
- **Read and write ports are separate traits.**

**✅ `cargo check -p domain` should succeed.** You have a compiling domain crate with zero external dependencies beyond `chrono` and `async-trait`.

---

## Phase 3: Application Crate

**Why second?** The application crate only depends on `domain`. It defines the unit-of-work port that persistence will implement. Build it before persistence so that the adapter knows what contract to fulfill.

### Step 7 — Define application errors

Create `crates/application/src/errors.rs`:

```rust
use std::fmt;

#[derive(Debug)]
pub struct NotFound { pub message: String }
impl fmt::Display for NotFound { /* ... */ }
impl std::error::Error for NotFound {}

#[derive(Debug)]
pub struct Forbidden { pub message: String }
impl fmt::Display for Forbidden { /* ... */ }
impl std::error::Error for Forbidden {}
```

### Step 8 — Define the Unit-of-Work port

Create `crates/application/src/uow_port.rs`. **This is the linchpin of the entire architecture.**

```rust
use async_trait::async_trait;
use domain::author::repository_port::{AuthorReadRepoPort, AuthorWriteRepoPort};

/// Transaction boundary for a single business operation.
#[async_trait]
pub trait UnitOfWorkPort: Send + Sync {
    fn author_ro(&self) -> &dyn AuthorReadRepoPort;
    fn author_rw(&self) -> &dyn AuthorWriteRepoPort;

    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn commit(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn rollback(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn release(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Factory for creating scoped UoW instances.
/// This trait is object-safe, enabling Arc<dyn WriteUnitOfWorkFactory> in the api crate.
pub trait WriteUnitOfWorkFactory: Send + Sync {
    fn create_ro(&self) -> Box<dyn UnitOfWorkPort>;
    fn create_rw(&self) -> Box<dyn UnitOfWorkPort>;
}
```

Key design decisions:
- **`WriteUnitOfWorkFactory` is a trait**, not a concrete type. This is what allows the `api` crate to use `Arc<dyn WriteUnitOfWorkFactory>` without knowing about the `persistence` crate.
- **The trait is object-safe** — methods take `&self` and return trait objects, not associated types or generics.
- **`create_ro` vs `create_rw`** — The factory produces separate instances for read-only vs read-write pools.

### Step 9 — Write the application service

Create `crates/application/src/author/queries.rs`:

```rust
use crate::errors::NotFound;
use crate::uow_port::UnitOfWorkPort;
use domain::author::entity::Author;

pub struct AuthorReadService {
    uow: Box<dyn UnitOfWorkPort>,
}

impl AuthorReadService {
    pub fn new(uow: Box<dyn UnitOfWorkPort>) -> Self {
        Self { uow }
    }

    pub async fn get_by_uuid(
        &mut self,
        author_uuid: &str,
    ) -> Result<Author, Box<dyn std::error::Error + Send + Sync>> {
        self.uow.start().await?;
        // ... business logic using self.uow.author_ro() ...
    }
}
```

Notice the imports: `crate::` for sibling modules within the application crate, `domain::` for cross-crate imports from the domain crate. No `persistence::`, no `api::` — the compiler prevents it.

**✅ `cargo check -p application` should succeed.**

---

## Phase 4: Persistence Crate

**Why third?** Persistence implements the ports defined in domain and application. Those ports must exist before you can `impl` them.

### Step 10 — Create the connection pool wrapper

Create `crates/persistence/src/database/sql/pool.rs` (unchanged from before).

### Step 11 — Create the DB row struct and mapper

Create `crates/persistence/src/author/mappers.rs`:

```rust
use domain::author::entity::Author;           // Cross-crate import from domain
use crate::struct_type::cache::StructTypeCache; // Sibling import within persistence

#[derive(Debug, sqlx::FromRow)]
pub struct DbAuthor {
    pub status_id: i32,    // Integer FK in the DB
    // ... other fields matching the DB schema
}

pub struct AuthorMappers { /* ... */ }

impl AuthorMappers {
    pub async fn db_to_domain(&self, db: &DbAuthor) -> Result<Author, ...> {
        // Resolve status_id → status string via cache
    }
}
```

### Step 12 — Create the repository adapter

Create `crates/persistence/src/author/read/repo.rs`:

```rust
use domain::author::entity::Author;
use domain::author::repository_port::AuthorReadRepoPort;
use crate::author::mappers::{AuthorMappers, DbAuthor};

pub struct SqlAuthorReadRepo { /* ... */ }

#[async_trait]
impl AuthorReadRepoPort for SqlAuthorReadRepo {
    // The trait is from the domain crate. The impl is in the persistence crate.
}
```

### Step 13 — Implement `UnitOfWorkPort` and `WriteUnitOfWorkFactory`

Create `crates/persistence/src/database/uow.rs`. The key change from a single-crate approach:

```rust
use application::uow_port::{WriteUnitOfWorkFactory, UnitOfWorkPort};

pub struct SqlWriteUnitOfWorkFactory {
    lifecycle: PoolLifecycle,
    repo_factory: SqlRepoFactory,  // Fully initialized, no Option, no deferred injection
}

impl SqlWriteUnitOfWorkFactory {
    pub fn new(lifecycle: PoolLifecycle, repo_factory: SqlRepoFactory) -> Self {
        Self { lifecycle, db: InMemoryDb::new(), repo_factory }
    }
}

/// SqlWriteUnitOfWorkFactory formally implements the application-layer trait.
impl WriteUnitOfWorkFactory for SqlWriteUnitOfWorkFactory {
    fn create_ro(&self) -> Box<dyn UnitOfWorkPort> {
        Box::new(SqlUnitOfWork { pool: self.lifecycle.ro_pool.pool().clone(), /* ... */ })
    }
    fn create_rw(&self) -> Box<dyn UnitOfWorkPort> {
        Box::new(SqlUnitOfWork { pool: self.lifecycle.rw_pool.pool().clone(), /* ... */ })
    }
}
```

**Why no deferred injection?** In a multi-crate workspace, the startup sequencing happens in the `server` crate. The `StructTypeCache` is hydrated *before* the `SqlRepoFactory` is created, which is before `SqlWriteUnitOfWorkFactory` is created. So the factory is always fully initialized at construction — no `Option`, no `set_repo_factory()`, no `RwLock`.

**✅ `cargo check -p persistence` should succeed** (if DATABASE_URL is set or sqlx prepare has been run).

---

## Phase 5: API Crate

**Why fourth?** The API crate depends on `domain` and `application`. It has **zero knowledge of persistence**. This is the most important boundary in the workspace.

### Step 14 — Create the dependencies struct

Create `crates/api/src/dependencies.rs`:

```rust
use std::sync::Arc;
use application::author::queries::AuthorReadService;
use application::uow_port::WriteUnitOfWorkFactory;

/// Note: no persistence imports possible. The compiler prevents it.
#[derive(Clone)]
pub struct AppDeps {
    pub write_uow_factory: Arc<dyn WriteUnitOfWorkFactory>,  // Trait object, not concrete type
}

impl AppDeps {
    pub fn get_author_read_service(&self) -> AuthorReadService {
        AuthorReadService::new(self.write_uow_factory.create_ro())
    }
}
```

**Key differences from a single-crate approach:**
- `Arc<dyn WriteUnitOfWorkFactory>` instead of `Arc<RwLock<SqlWriteUnitOfWorkFactory>>`.
- No `RwLock` needed — the factory is immutable after construction.
- No `async` on the getter methods — no lock to acquire.
- **Zero persistence imports.** The compiler guarantees this.

### Step 15 — Create DTOs and handlers

Create `crates/api/src/author/get.rs`:

```rust
use crate::dependencies::AppDeps;
use application::errors::{Forbidden, NotFound};
use domain::author::entity::Author;

pub async fn get_authors_handler(State(deps): State<AppDeps>) -> impl IntoResponse {
    let mut service = deps.get_author_read_service();  // No .await needed
    match service.list_all().await {
        Ok(authors) => { /* ... */ }
        Err(e) => write_error(e),
    }
}
```

### Step 16 — Create the router

Create `crates/api/src/router.rs` (route registration, unchanged in structure).

**✅ `cargo check -p api` should succeed.**

---

## Phase 6: Server Crate (Composition Root)

### Step 17 — Write the composition root

Create `crates/server/src/main.rs`:

```rust
use std::sync::Arc;
use application::uow_port::WriteUnitOfWorkFactory;
use persistence::database::sql::pool::PoolLifecycle;
use persistence::database::uow::{SqlRepoFactory, SqlWriteUnitOfWorkFactory};
use persistence::struct_type::cache::StructTypeCache;
use api::dependencies::AppDeps;
use api::router::new_router;

#[tokio::main]
async fn main() {
    // 1. Create connection pools
    let pool_lifecycle = PoolLifecycle::new(&ro_dsn, &rw_dsn).await.unwrap();
    pool_lifecycle.startup().await.unwrap();

    // 2. Hydrate lookup caches
    let cache = StructTypeCache::hydrate(pool_lifecycle.ro_pool.pool().clone()).await.unwrap();

    // 3. Create the fully-initialized UoW factory
    let repo_factory = SqlRepoFactory::new(cache);
    let write_uow_factory = SqlWriteUnitOfWorkFactory::new(pool_lifecycle.clone(), repo_factory);

    // 4. TYPE ERASURE — the most important line in the project
    let deps = AppDeps {
        write_uow_factory: Arc::new(write_uow_factory) as Arc<dyn WriteUnitOfWorkFactory>,
    };

    // 5. Build router and serve
    let app = new_router(deps);
    // ... axum::serve(listener, app) ...
}
```

**Step 4 is the boundary.** `Arc::new(write_uow_factory)` creates `Arc<SqlWriteUnitOfWorkFactory>` (a concrete persistence type). The `as Arc<dyn WriteUnitOfWorkFactory>` cast erases the concrete type. From this point on, the `api` crate (which receives `AppDeps`) can never reach back to `persistence`.

---

## Summary: Order of Operations

| Phase | Crate | Dependencies | Why This Order |
|-------|-------|--------------|----------------|
| 1 | Workspace scaffold | — | Need workspace manifest before anything compiles |
| 2 | **`domain`** | chrono, async-trait | Zero dependencies, everything else imports from here |
| 3 | **`application`** | domain | Depends only on domain; defines contracts for persistence |
| 4 | **`persistence`** | domain, application, sqlx | Implements the contracts from domain + application |
| 5 | **`api`** | domain, application, axum | Depends on inner layers only; cannot see persistence |
| 6 | **`server`** | all crates | Composition root; bridges persistence → api via type erasure |

The golden rule: **you should be able to `cargo check -p <crate>` after completing each phase.** If adding persistence requires changes to domain, something is wrong. If adding API handlers requires changes to application services, something is wrong. Each crate is additive.

The compiler enforces this. Not convention, not code review, not discipline — **the compiler.**
