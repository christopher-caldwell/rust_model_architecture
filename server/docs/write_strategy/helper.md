# UoW Execution Helper — Findings & Future Work

## Problem

Every write command in the application layer repeats the same lifecycle:

```rust
let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
let result = uow.<repo>().<op>().await.map_err(|e| anyhow::anyhow!(e)).context("…")?;
uow.commit().await.context("Failed to commit transaction")?;
Ok(result)
```

This is ~10 lines of plumbing per method. It will appear verbatim in every current and future command module.

## Recommended Solution

Add an extension trait on `WriteUnitOfWorkFactory` in `commands/shared.rs`:

```rust
use anyhow::Context;
use crate::ports::uow::{UnitOfWorkPort, WriteUnitOfWorkFactory};

pub trait UowFactoryExt: WriteUnitOfWorkFactory {
    async fn run<T, F>(&self, f: F) -> anyhow::Result<T>
    where
        F: AsyncFnOnce(&dyn UnitOfWorkPort) -> anyhow::Result<T>,
    {
        let uow = self.build().await.context("Failed to build unit of work")?;
        let result = f(uow.as_ref()).await?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }
}

impl<U: WriteUnitOfWorkFactory + ?Sized> UowFactoryExt for U {}
```

Re-export from `commands/mod.rs`:

```rust
pub use shared::UowFactoryExt;
```

### Call site (before)

```rust
pub async fn add_book(&self, payload: BookCreationPayload) -> anyhow::Result<Book> {
    let prepared = payload.prepare();
    let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
    let result = uow
        .book_write_repo()
        .create(&prepared)
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to insert book")?;
    uow.commit().await.context("Failed to commit transaction")?;
    Ok(result)
}
```

### Call site (after)

```rust
pub async fn add_book(&self, payload: BookCreationPayload) -> anyhow::Result<Book> {
    let prepared = payload.prepare();
    self.uow_factory
        .run(async |uow| {
            uow.book_write_repo()
                .create(&prepared)
                .await
                .map_err(anyhow::Error::msg)
                .context("Failed to insert book")
        })
        .await
}
```

Pre-transaction validation stays outside the closure — nothing changes there.

### Why this shape

| Alternative | Why rejected |
|---|---|
| Free function | Worse ergonomics; `run(&*self.uow_factory, …)` vs `self.uow_factory.run(…)` |
| Macro | Hides control flow; hurts IDE navigation |
| Generic base struct | Rust has no inheritance; forces coupling between command modules |

`?Sized` on the blanket impl is required to call `run` through `Arc<dyn WriteUnitOfWorkFactory>` (the field type used in all current command structs).

Requires Rust ≥ 1.85 for `AsyncFnOnce` in trait bounds (stabilised Feb 2025).

---

## Future Work

### 1. Repository error type

All write repo ports currently return `Result<T, String>`. This forces every call site to `.map_err(anyhow::Error::msg)`. Options, in order of preference:

- **Introduce a `RepoError` enum** (via `thiserror`) in the `domain` crate — gives typed errors that command handlers can match on, while remaining `anyhow`-compatible via `impl From<RepoError> for anyhow::Error`.
- **Change repo ports to `anyhow::Result<T>`** — simple, but couples the port definition to `anyhow` (minor issue since `anyhow` is already a dep of `application`).

### 2. Read-side helper

A symmetric `query<T, F>` helper on a read-unit-of-work factory (or on the individual read repos) would eliminate equivalent boilerplate on the query side once that layer grows.

### 3. Multi-repo transactions

The `run` closure already supports multiple repo calls in a single transaction — no new abstraction needed. Just call multiple repos inside the closure:

```rust
self.uow_factory.run(async |uow| {
    let loan = uow.loan_write_repo().create(&prepared_loan).await.map_err(anyhow::Error::msg)?;
    uow.book_copy_write_repo().update_status(copy_id, "on_loan").await.map_err(anyhow::Error::msg)?;
    Ok(loan)
}).await
```

### 4. Idempotency / optimistic locking

When operations need a version/etag check before updating, a `run_if<Guard>` variant could accept a guard closure that runs inside the transaction before the mutation. No action needed now.
