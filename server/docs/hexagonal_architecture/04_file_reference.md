# Hexagonal Architecture — File-by-File Reference

This document maps every file in the workspace to its architectural role and Cargo.toml enforcement.

---

## Workspace Structure

```
rust_server/
├── Cargo.toml                                            Workspace manifest
├── Makefile
├── .env
│
├── crates/
│   ├── domain/                                           CRATE 1 — deps: chrono, async-trait
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── author/
│   │       │   ├── mod.rs
│   │       │   ├── entity.rs                             Author domain entity
│   │       │   ├── repository_port.rs                    AuthorReadRepoPort, AuthorWriteRepoPort
│   │       │   └── exceptions.rs                         UserIsBannedException
│   │       └── user/
│   │           ├── mod.rs
│   │           ├── entity.rs                             User domain entity
│   │           └── repository_port.rs                    UserReadRepoPort, UserWriteRepoPort
│   │
│   ├── application/                                      CRATE 2 — deps: domain
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── errors.rs                                 NotFound, Forbidden
│   │       ├── uow_port.rs                               UnitOfWorkPort, WriteUnitOfWorkFactory
│   │       ├── author/
│   │       │   ├── mod.rs
│   │       │   ├── queries.rs                            AuthorReadService
│   │       │   └── commands.rs                           AuthorWriteService
│   │       └── user/
│   │           ├── mod.rs
│   │           └── queries.rs                            UserReadService
│   │
│   ├── persistence/                                      CRATE 3 — deps: domain, application, sqlx
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── database/
│   │       │   ├── mod.rs
│   │       │   ├── sql/
│   │       │   │   ├── mod.rs
│   │       │   │   └── pool.rs                           SqlPool, PoolLifecycle
│   │       │   ├── in_memory/
│   │       │   │   ├── mod.rs
│   │       │   │   └── store.rs                          InMemoryDb, InMemoryUserReadRepo
│   │       │   └── uow.rs                               SqlRepoFactory, SqlUnitOfWork, SqlWriteUnitOfWorkFactory
│   │       ├── author/
│   │       │   ├── mod.rs
│   │       │   ├── mappers.rs                            DbAuthor, AuthorMappers
│   │       │   ├── read/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── repo.rs                           SqlAuthorReadRepo
│   │       │   │   ├── queries.sql                       Reference SQL (documentation)
│   │       │   │   └── queries/
│   │       │   │       ├── get_author_by_uuid.sql        query_file_as! source
│   │       │   │       └── list_authors.sql              query_file_as! source
│   │       │   └── write.rs                              SqlAuthorWriteRepo (stub)
│   │       └── struct_type/
│   │           ├── mod.rs
│   │           └── cache.rs                              StructType, StructTypeCache
│   │
│   ├── api/                                              CRATE 4 — deps: domain, application (NO persistence)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── dependencies.rs                           AppDeps (Arc<dyn WriteUnitOfWorkFactory>)
│   │       ├── router.rs                                 new_router()
│   │       ├── author/
│   │       │   ├── mod.rs
│   │       │   └── get.rs                                AuthorDto, handlers
│   │       └── user/
│   │           ├── mod.rs
│   │           └── get.rs                                UserDto, handlers
│   │
│   └── server/                                           CRATE 5 — deps: ALL (composition root)
│       ├── Cargo.toml
│       └── src/
│           └── main.rs                                   Bootstrap + entry point
│
└── docs/
    └── hexagonal_architecture/
        ├── 01_architecture_overview.md
        ├── 02_implementation_guide.md
        ├── 03_patterns_and_rules.md
        └── 04_file_reference.md
```

---

## Cargo.toml Dependency Map

This is the definitive reference for what each crate is allowed to import:

| Crate | Internal Dependencies | External Dependencies |
|-------|----------------------|----------------------|
| `domain` | — | `async-trait`, `chrono` |
| `application` | `domain` | `async-trait`, `chrono`, `uuid` |
| `persistence` | `domain`, `application` | `async-trait`, `sqlx`, `chrono`, `uuid`, `tokio` |
| `api` | `domain`, `application` | `axum`, `serde`, `serde_json` |
| `server` | `domain`, `application`, `persistence`, `api` | `axum`, `tokio`, `dotenvy` |

---

## Import Map (every file, every import)

### `crates/server/src/main.rs`
```
← std::{env, sync::Arc}
← tokio::signal
← application::uow_port::WriteUnitOfWorkFactory        (cross-crate)
← persistence::database::sql::pool::PoolLifecycle  (cross-crate)
← persistence::database::uow::{SqlRepoFactory, SqlWriteUnitOfWorkFactory}  (cross-crate)
← persistence::struct_type::cache::StructTypeCache  (cross-crate)
← api::dependencies::AppDeps                        (cross-crate)
← api::router::new_router                           (cross-crate)
← dotenvy
```

### `crates/domain/src/author/entity.rs`
```
← chrono::{DateTime, Utc}
```
*No crate-internal imports. No cross-crate imports.*

### `crates/domain/src/author/repository_port.rs`
```
← async_trait::async_trait
← super::entity::Author                             (same crate)
```

### `crates/domain/src/author/exceptions.rs`
```
← std::fmt
```

### `crates/domain/src/user/entity.rs`
```
← chrono::{DateTime, Utc}
```

### `crates/domain/src/user/repository_port.rs`
```
← async_trait::async_trait
← super::entity::User                               (same crate)
```

### `crates/application/src/errors.rs`
```
← std::fmt
```

### `crates/application/src/uow_port.rs`
```
← async_trait::async_trait
← domain::author::repository_port::*                 (cross-crate → domain)
← domain::user::repository_port::*                   (cross-crate → domain)
```

### `crates/application/src/author/queries.rs`
```
← crate::errors::NotFound                            (same crate)
← crate::uow_port::UnitOfWorkPort                    (same crate)
← domain::author::entity::Author                     (cross-crate → domain)
```

### `crates/application/src/author/commands.rs`
```
← crate::uow_port::UnitOfWorkPort                    (same crate)
← domain::author::entity::Author                     (cross-crate → domain)
← uuid, chrono                                       (external)
```

### `crates/application/src/user/queries.rs`
```
← crate::errors::NotFound                            (same crate)
← crate::uow_port::UnitOfWorkPort                    (same crate)
← domain::user::entity::User                         (cross-crate → domain)
```

### `crates/persistence/src/database/sql/pool.rs`
```
← sqlx::postgres::PgPool                             (external)
```

### `crates/persistence/src/database/in_memory/store.rs`
```
← async_trait, chrono, std, tokio                     (external)
← domain::user::entity::User                         (cross-crate → domain)
← domain::user::repository_port::*                   (cross-crate → domain)
```

### `crates/persistence/src/database/uow.rs`
```
← async_trait, sqlx                                   (external)
← application::uow_port::{WriteUnitOfWorkFactory, UnitOfWorkPort}  (cross-crate → application)
← domain::author::repository_port::*                  (cross-crate → domain)
← domain::user::repository_port::*                    (cross-crate → domain)
← crate::author::mappers::AuthorMappers               (same crate)
← crate::author::read::repo::SqlAuthorReadRepo        (same crate)
← crate::author::write::SqlAuthorWriteRepo             (same crate)
← crate::database::in_memory::store::*                (same crate)
← crate::database::sql::pool::PoolLifecycle            (same crate)
← crate::struct_type::cache::StructTypeCache           (same crate)
```

### `crates/persistence/src/author/mappers.rs`
```
← chrono                                              (external)
← domain::author::entity::Author                      (cross-crate → domain)
← crate::struct_type::cache::StructTypeCache           (same crate)
```

### `crates/persistence/src/author/read/repo.rs`
```
← async_trait, sqlx                                    (external)
← domain::author::entity::Author                      (cross-crate → domain)
← domain::author::repository_port::AuthorReadRepoPort  (cross-crate → domain)
← crate::author::mappers::{AuthorMappers, DbAuthor}   (same crate)
```

### `crates/persistence/src/author/write.rs`
```
← domain::author::repository_port::AuthorWriteRepoPort  (cross-crate → domain)
```

### `crates/persistence/src/struct_type/cache.rs`
```
← chrono, sqlx, std, tokio                             (external)
```

### `crates/api/src/dependencies.rs`
```
← std::sync::Arc
← application::author::queries::AuthorReadService      (cross-crate → application)
← application::uow_port::WriteUnitOfWorkFactory             (cross-crate → application)
← application::user::queries::UserReadService           (cross-crate → application)
```
*Zero persistence imports. The compiler enforces this.*

### `crates/api/src/router.rs`
```
← axum::{routing::get, Router}                         (external)
← crate::author::get::*                                (same crate)
← crate::dependencies::AppDeps                         (same crate)
← crate::user::get::*                                  (same crate)
```

### `crates/api/src/author/get.rs`
```
← axum::*, serde::Serialize                             (external)
← crate::dependencies::AppDeps                         (same crate)
← application::errors::{Forbidden, NotFound}            (cross-crate → application)
← domain::author::entity::Author                       (cross-crate → domain)
```

### `crates/api/src/user/get.rs`
```
← axum::*, serde::Serialize                             (external)
← crate::dependencies::AppDeps                         (same crate)
← application::errors::{Forbidden, NotFound}            (cross-crate → application)
← domain::user::entity::User                           (cross-crate → domain)
```

---

## Architectural Audit Results

After reviewing every file and import in the workspace:

- ✅ **`domain` crate has zero internal cross-crate imports.** Only `chrono` and `async-trait`.
- ✅ **`application` crate imports only from `domain`.** No persistence, no API.
- ✅ **`persistence` crate imports only from `domain` and `application`.** No API.
- ✅ **`api` crate imports only from `domain` and `application`.** No persistence. **Compiler-enforced.**
- ✅ **`server` crate imports from all four.** This is the composition root — it must.
- ✅ **`SqlWriteUnitOfWorkFactory` implements `WriteUnitOfWorkFactory` trait** — enabling the `Arc::new(factory) as Arc<dyn WriteUnitOfWorkFactory>` type erasure.
- ✅ **`AppDeps` stores `Arc<dyn WriteUnitOfWorkFactory>`** — not a concrete type, not behind `RwLock`.
- ✅ **DTOs (`Serialize`) exist only in the api crate.** `serde` is not a domain dependency.
- ✅ **DB row structs (`FromRow`) exist only in the persistence crate.** `sqlx` is not a domain dependency.
- ✅ **All boundaries are enforced by `Cargo.toml` dependency lists**, not by convention.
