.PHONY: dev build run typecheck lint lint-fix fmt test check-sanity clippy

# Type check all crates
typecheck:
	cargo check --workspace

# Lint with clippy (Rust's official linter)
clippy:
	cargo clippy --workspace -- -D warnings

# Format code
fmt:
	cargo fmt --all

# Format check (non-destructive)
fmt-check:
	cargo fmt --all -- --check

# Run tests
test:
	cargo test --workspace

# Cache sqlx query metadata for offline compilation.
#
# The query_file_as! macros (used in repo.rs) verify SQL against a live database
# at compile time. This means `cargo check` / `cargo build` normally requires
# DATABASE_URL to be set and the database to be reachable.
#
# `cargo sqlx prepare` connects to the database ONCE, validates every query macro
# in the codebase, and saves the results to a `.sqlx/` directory. After this:
#   - Compilation no longer needs a live database
#   - CI can build without database access
#   - The `.sqlx/` directory should be committed to version control
#
# Re-run this whenever you change a query_file_as! query or the DB schema.
prepare:
	cargo sqlx prepare

# Full sanity check: typecheck + clippy + fmt-check
check-sanity: typecheck clippy fmt-check
	@echo "✅ All checks passed"
