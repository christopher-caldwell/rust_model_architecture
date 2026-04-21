# Pattern: In-Memory Status/Dictionary Lookups vs SQL JOINs

## Overview
Moving dictionary/status lookups from SQL `JOIN`s to an in-memory application cache (using `RwLock` in Rust) is a common performance optimization. It removes database overhead for relatively static data and localizes it to the application's memory.

## Implementation Flow

### 1. Update the SQL Query
Remove the `JOIN` from your query and instead select the raw foreign key ID (e.g., `status_id`).

```sql
-- Before
SELECT ci.id, st.att_pub_ident "status"
FROM contact_inquiry ci
JOIN struct_type st ON ci.status_id = st.struct_type_id;

-- After
SELECT ci.id, ci.status_id
FROM contact_inquiry ci;
```

### 2. Update the Database Row Struct
Update the `DbRow` struct in Rust to reflect the raw integer ID instead of the joined string.

```rust
pub struct ContactInquiryDbRow {
    pub contact_inquiry_id: i16,
    pub status_id: i16, // Changed from `status: String`
    // ... other fields
}
```

### 3. Inject the Cache Repository (Dependency Injection)
To perform the lookup during data retrieval, the Read Repository needs access to the Status Repository (which holds the in-memory cache).

```rust
use std::sync::Arc;

pub struct ContactInquiryReadRepoSql {
    pub pool: sqlx::PgPool,
    // Inject the status repo to access the cache
    pub status_repo: Arc<dyn ContactInquiryStatusRepoPort>, 
}
```

### 4. Async Manual Domain Mapping
Because fetching from the cache requires an `.await` (to acquire an async `RwLock` or fetch from the DB on cache miss), you can no longer use Rust's synchronous `Into` or `From` traits for automatic mapping (e.g., `row.map(ContactInquiryDbRow::into)`).

You must map the DB row to the Domain Entity manually within the async context:

```rust
async fn get_contact_inquiry_by_id(&self, id: i16) -> Result<Option<ContactInquiry>, String> {
    let row = sqlx::query_file_as!(/* ... */)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

    match row {
        Some(db_row) => {
            // 1. Await the cache lookup
            let status_string = self.status_repo
                .get_status_ident_by_id(&db_row.status_id)
                .await?;

            // 2. Map the domain entity manually
            let domain_entity = ContactInquiry {
                status: status_string,
                contact_inquiry_id: db_row.contact_inquiry_id,
                first_name: db_row.first_name,
                // ... copy the rest of the fields
            };

            Ok(Some(domain_entity))
        }
        None => Ok(None)
    }
}
```

## Feasibility Analysis & Trade-offs

### Pros
* **Faster Database Queries:** Removes `JOIN`s, reducing database load and speeding up query execution times.
* **Separation of Concerns:** The `DbRow` struct maps exactly to the database table, while the domain mapping assembles the related concepts using application logic.

### Cons / Architectural Impacts
* **Loss of `Into` Convenience:** You must write manual mapping code for any entity that requires an async cache lookup.
* **Increased Coupling:** Repositories now depend on each other (e.g., `ReadRepo` depends on `StatusRepo`). Dependency injection must be wired up in the server initialization.
* **The N+1 Problem for List Queries:** If executing a `get_all` query returning 100 rows, mapping the domain entities requires 100 separate `.await` calls to the cache in a loop. While an in-memory `RwLock` is extremely fast, acquiring and releasing locks repeatedly is slightly more CPU-intensive than executing a single SQL `JOIN`.
