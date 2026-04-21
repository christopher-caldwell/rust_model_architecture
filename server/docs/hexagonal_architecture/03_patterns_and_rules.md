# Hexagonal Architecture — Patterns & Rules Reference

This document is a precise reference of every repeatable pattern and import rule in the architecture. Use it as a checklist when adding new entities, services, or endpoints.

---

## 1. Import Rules (Compiler-Enforced via Cargo.toml)

These rules are **enforced by the Rust compiler**. Each crate's `Cargo.toml` declares exactly which other crates it may import. Attempting to violate any of these rules produces a compile error, not a warning.

| Crate (`Cargo.toml`) | Can import from | Cannot import from |
|-----------------------|-----------------|--------------------|
| **`domain`** | (nothing internal) | `application`, `persistence`, `api`, `server` |
| **`application`** | `domain` | `persistence`, `api`, `server` |
| **`persistence`** | `domain`, `application` | `api`, `server` |
| **`api`** | `domain`, `application` | `persistence`, `server` |
| **`server`** | `domain`, `application`, `persistence`, `api` | — |

### How the compiler enforces this

If a developer adds `use persistence::database::uow::SqlWriteUnitOfWorkFactory;` to any file in the `api` crate, they get:

```
error[E0433]: failed to resolve: use of undeclared crate or module `persistence`
```

This is **not** a lint, not a warning — it's a hard compilation failure. The only way to bypass it is to add `persistence` as a dependency in `api/Cargo.toml`, which would be caught in code review.

### Import path conventions within the workspace

| Where you are | Importing from same crate | Importing from another crate |
|---------------|--------------------------|------------------------------|
| `crates/application/src/author/queries.rs` | `use crate::errors::NotFound;` | `use domain::author::entity::Author;` |
| `crates/persistence/src/database/uow.rs` | `use crate::author::mappers::AuthorMappers;` | `use application::uow_port::UnitOfWorkPort;` |
| `crates/api/src/author/get.rs` | `use crate::dependencies::AppDeps;` | `use application::errors::NotFound;` |

---

## 2. Entity Pattern

**Location**: `crates/domain/src/{entity_name}/entity.rs`

```rust
use chrono::{DateTime, Utc};

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

### Rules
- Derive only `Debug` and `Clone`. No `Serialize`, `Deserialize`, or `FromRow`.
- The domain crate's `Cargo.toml` includes only `chrono` and `async-trait` — `serde` and `sqlx` are physically absent.
- All fields use business-language types (`String` for status, not `i32`).

---

## 3. Repository Port Pattern

**Location**: `crates/domain/src/{entity_name}/repository_port.rs`

```rust
use async_trait::async_trait;
use super::entity::Author;

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

pub trait AuthorWriteRepoPort: Send + Sync {}
```

### Rules
- Always `#[async_trait]` with `Send + Sync` supertrait bounds.
- Error type is always `Box<dyn std::error::Error + Send + Sync>`.
- Read and write ports are **separate traits**.
- Only references domain entities — no DTOs, no DB row structs.

---

## 4. Unit-of-Work Port Pattern

**Location**: `crates/application/src/uow_port.rs`

```rust
#[async_trait]
pub trait UnitOfWorkPort: Send + Sync {
    fn author_ro(&self) -> &dyn AuthorReadRepoPort;
    fn author_rw(&self) -> &dyn AuthorWriteRepoPort;

    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn commit(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn rollback(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn release(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Object-safe factory trait — enables Arc<dyn WriteUnitOfWorkFactory> in the api crate.
pub trait WriteUnitOfWorkFactory: Send + Sync {
    fn create_ro(&self) -> Box<dyn UnitOfWorkPort>;
    fn create_rw(&self) -> Box<dyn UnitOfWorkPort>;
}
```

### Rules
- `WriteUnitOfWorkFactory` **must be a trait** (not a concrete type) so the `api` crate can use `Arc<dyn WriteUnitOfWorkFactory>` without depending on the `persistence` crate.
- The trait must be **object-safe** — no generics, no `Self` in return types.
- Every entity with a repo port gets two accessors: `_ro()` and `_rw()`.

---

## 5. Application Service Pattern

**Location**: `crates/application/src/{entity_name}/queries.rs` or `commands.rs`

```rust
use crate::errors::NotFound;           // Same crate
use crate::uow_port::UnitOfWorkPort;   // Same crate
use domain::author::entity::Author;     // Cross-crate

pub struct AuthorReadService {
    uow: Box<dyn UnitOfWorkPort>,
}

impl AuthorReadService {
    pub fn new(uow: Box<dyn UnitOfWorkPort>) -> Self {
        Self { uow }
    }

    pub async fn get_by_uuid(&mut self, id: &str)
        -> Result<Author, Box<dyn std::error::Error + Send + Sync>>
    {
        self.uow.start().await?;
        match self.uow.author_ro().get_by_uuid(id).await {
            Ok(Some(entity)) => { self.uow.commit().await?; Ok(entity) }
            Ok(None) => {
                let _ = self.uow.rollback().await;
                Err(Box::new(NotFound { message: format!("...") }))
            }
            Err(e) => { let _ = self.uow.rollback().await; Err(e) }
        }
    }
}
```

### Rules
- Takes `Box<dyn UnitOfWorkPort>` — never a concrete factory or pool.
- Returns domain entities, never DTOs or DB types.
- **Queries** use `write_uow_factory.create_ro()`. **Commands** use `create_rw()`.

---

## 6. DB Row Struct & Mapper Pattern

**Location**: `crates/persistence/src/{entity_name}/mappers.rs`

```rust
use domain::author::entity::Author;               // Cross-crate
use crate::struct_type::cache::StructTypeCache;    // Same crate

#[derive(Debug, sqlx::FromRow)]
pub struct DbAuthor {
    pub status_id: i32,    // Integer FK, not a string
    // ...
}

#[derive(Clone)]
pub struct AuthorMappers { cache: StructTypeCache }

impl AuthorMappers {
    pub async fn db_to_domain(&self, db: &DbAuthor) -> Result<Author, ...> {
        // Resolve status_id → status string via cache
    }
}
```

### Rules
- `DbAuthor` derives `sqlx::FromRow` — only possible because `sqlx` is in the persistence crate's `Cargo.toml`.
- Domain `Author` does not derive `FromRow` — `sqlx` is not in the domain crate's `Cargo.toml`.
- The compiler guarantees this separation.

---

## 7. Repository Adapter Pattern

**Location**: `crates/persistence/src/{entity_name}/read/repo.rs`

```rust
use domain::author::entity::Author;
use domain::author::repository_port::AuthorReadRepoPort;
use crate::author::mappers::{AuthorMappers, DbAuthor};

pub struct SqlAuthorReadRepo { pool: PgPool, mappers: AuthorMappers }

#[async_trait]
impl AuthorReadRepoPort for SqlAuthorReadRepo {
    // Trait from domain crate, impl in persistence crate
}
```

---

## 8. UoW Adapter & Factory Pattern

**Location**: `crates/persistence/src/database/uow.rs`

```rust
use application::uow_port::{WriteUnitOfWorkFactory, UnitOfWorkPort};

pub struct SqlWriteUnitOfWorkFactory {
    lifecycle: PoolLifecycle,
    db: InMemoryDb,
    repo_factory: SqlRepoFactory,  // Fully initialized — no Option, no deferred injection
}

impl SqlWriteUnitOfWorkFactory {
    pub fn new(lifecycle: PoolLifecycle, repo_factory: SqlRepoFactory) -> Self {
        Self { lifecycle, db: InMemoryDb::new(), repo_factory }
    }
}

/// The concrete factory implements the abstract application-layer trait.
impl WriteUnitOfWorkFactory for SqlWriteUnitOfWorkFactory {
    fn create_ro(&self) -> Box<dyn UnitOfWorkPort> { /* ... */ }
    fn create_rw(&self) -> Box<dyn UnitOfWorkPort> { /* ... */ }
}
```

### Rules
- `SqlWriteUnitOfWorkFactory` formally `impl WriteUnitOfWorkFactory` so it can be cast to `Arc<dyn WriteUnitOfWorkFactory>`.
- The factory is fully initialized at construction — no `Option<SqlRepoFactory>`, no `set_repo_factory()`, no `RwLock`.
- Repos are `Option<T>`, initialized in `start()`, set to `None` in `release()`.

---

## 9. DTO & Handler Pattern

**Location**: `crates/api/src/{entity_name}/get.rs`

```rust
use crate::dependencies::AppDeps;
use application::errors::{Forbidden, NotFound};
use domain::author::entity::Author;

#[derive(Serialize)]
pub struct AuthorDto { /* ... */ }

pub async fn get_authors_handler(State(deps): State<AppDeps>) -> impl IntoResponse {
    let mut service = deps.get_author_read_service();  // Not async — no lock
    match service.list_all().await {
        Ok(entities) => { /* ... */ }
        Err(e) => write_error(e),
    }
}
```

### Rules
- DTOs derive `Serialize` — possible because `serde` is in the api crate's `Cargo.toml`.
- Domain entities do NOT derive `Serialize` — the compiler enforces this.
- `deps.get_author_read_service()` is **synchronous** (no `.await`) because there's no `RwLock` to acquire.

---

## 10. Composition Root / Type-Erasure Pattern

**Location**: `crates/server/src/main.rs`

```rust
use application::uow_port::WriteUnitOfWorkFactory;
use persistence::database::uow::{SqlRepoFactory, SqlWriteUnitOfWorkFactory};
use api::dependencies::AppDeps;

// Fully initialize the factory BEFORE creating AppDeps
let write_uow_factory = SqlWriteUnitOfWorkFactory::new(pool_lifecycle, repo_factory);

// TYPE ERASURE — concrete persistence type becomes abstract application trait
let deps = AppDeps {
    write_uow_factory: Arc::new(write_uow_factory) as Arc<dyn WriteUnitOfWorkFactory>,
};
```

### Why this works
- `SqlWriteUnitOfWorkFactory` (persistence crate) implements `WriteUnitOfWorkFactory` (application crate).
- `Arc::new(write_uow_factory)` creates `Arc<SqlWriteUnitOfWorkFactory>`.
- `as Arc<dyn WriteUnitOfWorkFactory>` performs unsized coercion, erasing the concrete type.
- `AppDeps` stores `Arc<dyn WriteUnitOfWorkFactory>` — it cannot see the concrete type.

---

## 11. Adding a New Entity — Checklist

When you need to add a new entity (e.g., `Book`), follow this exact checklist:

### `domain` crate
- [ ] Create `crates/domain/src/book/entity.rs` with the entity struct
- [ ] Create `crates/domain/src/book/repository_port.rs` with `BookReadRepoPort` and `BookWriteRepoPort`
- [ ] Create `crates/domain/src/book/mod.rs` re-exporting modules
- [ ] Add `pub mod book;` to `crates/domain/src/lib.rs`

### `application` crate
- [ ] Add `fn book_ro(&self) -> &dyn BookReadRepoPort;` to `UnitOfWorkPort`
- [ ] Add `fn book_rw(&self) -> &dyn BookWriteRepoPort;` to `UnitOfWorkPort`
- [ ] Create `crates/application/src/book/queries.rs` with `BookReadService`
- [ ] Create `crates/application/src/book/commands.rs` with `BookWriteService` (if needed)
- [ ] Create `crates/application/src/book/mod.rs`
- [ ] Add `pub mod book;` to `crates/application/src/lib.rs`

### `persistence` crate
- [ ] Create `crates/persistence/src/book/mappers.rs` with `DbBook` and `BookMappers`
- [ ] Create `crates/persistence/src/book/read/repo.rs` with `SqlBookReadRepo` implementing `BookReadRepoPort`
- [ ] Create `crates/persistence/src/book/write.rs` with `SqlBookWriteRepo` implementing `BookWriteRepoPort`
- [ ] Add `book_ro: Option<SqlBookReadRepo>` and `book_rw: Option<SqlBookWriteRepo>` to `SqlUnitOfWork`
- [ ] Implement `book_ro()` and `book_rw()` on `SqlUnitOfWork`
- [ ] Add repo construction in `SqlUnitOfWork::start()`
- [ ] Add cleanup in `SqlUnitOfWork::release()`
- [ ] Add `pub fn book_read(&self, pool: PgPool) -> SqlBookReadRepo` to `SqlRepoFactory`
- [ ] Add `pub mod book;` to `crates/persistence/src/lib.rs`

### `api` crate
- [ ] Create `crates/api/src/book/get.rs` with `BookDto`, `to_dto`, handlers, and `write_error`
- [ ] Create `crates/api/src/book/mod.rs`
- [ ] Add routes in `crates/api/src/router.rs`
- [ ] Add `pub fn get_book_read_service(&self) -> BookReadService` to `AppDeps`
- [ ] Add `pub mod book;` to `crates/api/src/lib.rs`

### Verify
- [ ] `cargo check -p domain` passes
- [ ] `cargo check -p application` passes
- [ ] `cargo check -p api` passes
- [ ] `cargo check -p persistence` passes
- [ ] `cargo check -p server` passes
