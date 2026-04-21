# Raw SQL with sqlx in the Rust Server

## Overview

This document explains how raw SQL queries are used with [sqlx](https://github.com/launchbadge/sqlx) in the Rust hexagonal architecture server, and how this approach compares to the codegen approaches used in the Go (SQLC) and Node.js (pgTyped) servers.

## Approach: Inline SQL via `sqlx::query_as`

Unlike SQLC (Go) and pgTyped (Node.js), which generate Rust/Go/TS code from `.sql` files at build time, **sqlx uses raw SQL strings directly in Rust source code** with compile-time verification.

### How It Works

1. **SQL is written inline** in the repository implementation:

```rust
let row: Option<DbAuthor> = sqlx::query_as(
    "SELECT author_id, author_uuid, dt_created, dt_modified, status_id, first_name, last_name
     FROM author WHERE author_uuid = $1"
)
    .bind(author_uuid)
    .fetch_optional(&self.pool)
    .await?;
```

2. **`sqlx::FromRow` derive macro** maps DB columns to Rust struct fields:

```rust
#[derive(sqlx::FromRow)]
pub struct DbAuthor {
    pub author_id: i32,
    pub author_uuid: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status_id: i32,
    pub first_name: String,
    pub last_name: String,
}
```

3. **Type-safe parameter binding** via `.bind()` — sqlx infers types from the Rust values.

### Reference SQL Files

We maintain `.sql` files (e.g., `src/persistence/author/read/queries.sql`) alongside the Rust repos for **documentation and cross-server parity**. These files contain the same queries used in Go (SQLC) and Node.js (pgTyped), making it easy to compare implementations across all four servers.

These SQL files are **not consumed by any tooling** in the Rust server — they serve purely as a reference to ensure all servers execute identical queries.

## Comparison with Other Servers

| Aspect | Python (SQLC) | Node.js (pgTyped) | Go (SQLC) | Rust (sqlx) |
|---|---|---|---|---|
| **SQL Location** | `.sql` files | `.sql` files | `.sql` files | Inline in `.rs` + reference `.sql` |
| **Code Generation** | Yes (build-time) | Yes (watch-mode daemon) | Yes (build-time) | No codegen needed |
| **Type Safety** | Generated Python types | Generated TS types | Generated Go types | `FromRow` derive + compile-time |
| **Params** | `$1`, `$2` positional | `:param_name!` named | `$1`, `$2` positional | `$1`, `$2` + `.bind()` |
| **Return Type** | Single/list via annotation | Always array (`rows[0]`) | `:one` / `:many` annotation | `fetch_one` / `fetch_all` / `fetch_optional` |

## Why This Approach

1. **No build step required**: sqlx works at compile time with the `query!` macro (which checks SQL against a live DB), or at runtime with `query_as` (which we use here). This avoids needing separate codegen tooling.

2. **Idiomatic Rust**: The `FromRow` derive macro is the standard sqlx pattern. It maps snake_case DB columns directly to snake_case Rust struct fields with no extra configuration.

3. **Same SQL everywhere**: The actual SQL queries are identical across all four servers. The only difference is parameter syntax (`$1` vs `:param_name!`), which is a dialect difference between Postgres-native (used by SQLC, sqlx) and pgTyped.

4. **Compile-time checking available**: For production use, sqlx provides `sqlx::query!` which verifies queries against a live database at compile time. We use `sqlx::query_as` here to avoid requiring a live DB connection during compilation, but the queries are structured to be trivially migratable to `query!`.

## sqlx Fetch Methods

| Method | Returns | Equivalent |
|---|---|---|
| `fetch_one` | Single row (error if not found) | SQLC `:one` |
| `fetch_optional` | `Option<Row>` | Check `rows[0]` in pgTyped |
| `fetch_all` | `Vec<Row>` | SQLC `:many` |

## File Layout

```
src/persistence/author/
├── mappers.rs              # DbAuthor struct + AuthorMappers (with FromRow derive)
├── read/
│   ├── queries.sql         # Reference SQL (documentation only, not consumed by tooling)
│   ├── repo.rs             # SqlAuthorReadRepo (inline sqlx queries)
│   └── mod.rs
├── write.rs                # Stub write repo
└── mod.rs
```

This mirrors the exact same directory structure as Go, Node.js, and Python, maintaining cross-server consistency.
