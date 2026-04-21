# Reading Inside a Transaction: Why One Unified Read Repo Is Difficult

## The Goal

Ideally, we'd define each read repository once — a single struct, a single `impl` block — and use it both for standalone reads (against a connection pool) and for reads within a write transaction (inside a Unit of Work). This would eliminate duplication and keep the persistence layer clean.

## Why It's Difficult in sqlx

### The Executor Trait

In sqlx, both `PgPool` and `Transaction<'_, Postgres>` implement the `Executor` trait, which is what query methods like `.fetch_optional()`, `.fetch_all()`, and `.fetch_one()` accept. At first glance, this seems like it should make unification straightforward — just accept any `Executor`.

The problem is that `Executor` carries a lifetime parameter and is designed to be consumed (or mutably borrowed) per query execution. This makes it impossible to store as a trait object (`dyn Executor`) inside a struct. Rust's type system requires that the concrete executor type be known at compile time when calling sqlx's query macros.

### What This Means in Practice

A read repo that holds a `PgPool` calls:
```rust
sqlx::query_file_as!(Row, "query.sql", ...).fetch_optional(&self.pool).await
```

A read repo that holds a transaction calls:
```rust
let mut guard = self.tx.lock().await;
let tx = guard.as_deref_mut().unwrap();
sqlx::query_file_as!(Row, "query.sql", ...).fetch_optional(&mut **tx).await
```

The call sites are different types — `&PgPool` vs `&mut PgConnection` (via deref). There is no single type you can store in a struct field that satisfies both without branching at runtime.

### Approaches Considered

#### 1. Enum-Based Executor with Macro

Store an enum (`Pool(PgPool) | Transaction(Arc<Mutex<Option<Transaction>>>)`) in the struct and use a private macro to hide the match in each method. This achieves one struct and one `impl` block, but each method still contains a hidden branch. The macro keeps method bodies clean:

```rust
async fn get_by_ident(&self, ident: &str) -> Result<Option<Entity>, String> {
    let query = sqlx::query_file_as!(Row, "query.sql", ident);
    let row = execute!(self, query, fetch_optional)?;
    Ok(row.map(Into::into))
}
```

**Tradeoff:** One struct, clean method bodies, but the enum and macro are internal complexity that every future read repo must adopt.

#### 2. Generic Struct with Executor Type Parameter

Make the struct generic over the executor:

```rust
pub struct ContactInquiryReadRepo<E> {
    executor: E,
}
```

**Problem:** `E` would need to implement `Executor`, but `Executor` is consumed on use and has lifetime constraints that make this impractical for a stored field. The struct can't call `.fetch_optional(&self.executor)` more than once because `Executor` takes `self` by value for some implementors.

#### 3. Callback/Closure Injection

Store a closure that provides an executor on demand.

**Problem:** Over-engineered, lifetime issues with async closures, and the closure signature would need to be generic over the executor's lifetime — which circles back to the same fundamental problem.

## Current Decision

We chose to **not** duplicate read repos. The Unit of Work only exposes write repositories. All reads go through the standalone pool-based `ContactInquiryReadRepoSql`. This means:

- Reads within a command handler do **not** see uncommitted writes from the same transaction
- This is acceptable for our current use cases (we write and commit; we don't need to read back within the same transaction)
- Write repos can still read within their own transaction when needed (e.g., a `RETURNING *` clause in an INSERT/UPDATE)

## Future: When Transactional Reads Become Necessary

If a use case arises where a command must read its own uncommitted writes (e.g., insert a record, then query it back within the same transaction to compute something), the enum + macro approach from option 1 above is the most practical path forward. It would look like:

1. Define a `DbExecutor` enum in a shared module
2. Each read repo holds a `DbExecutor` instead of a raw `PgPool`
3. A crate-level `execute!` macro handles the match transparently
4. Convenience constructors (`from_pool`, `from_tx`) keep the wiring clean
5. The UOW factory constructs the read repo with `DbExecutor::Transaction(tx.clone())`
6. `main.rs` constructs the standalone read repo with `DbExecutor::Pool(pool)`

This is a mechanical refactor that can be done per-entity when the need arises without changing the domain or application layers.
