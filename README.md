# [ohkami](https://github.com/kana-rus/ohkami) - Intuitive and Declarative Web Framework for Rust

## Description

> Build web app in intuitive and declarative code
> - *macro-less and type-safe* APIs for intuitive and declarative code
> - *multi runtime* support：`tokio`, `async-std`

- [User Guide](https://docs.rs/ohkami/latest/ohkami/)
- [API Documentation](https://docs.rs/ohkami/latest/ohkami/)
- Cargo package: [ohkami](https://crates.io/crates/ohkami)

## Database

PostgreSQL

-  Raw using [sqlx](https://github.com/launchbadge/sqlx)

## Test URLs

### Test 1: JSON Encoding

    http://localhost:8000/json

### Test 2: Single Row Query

    http://localhost:8000/db

### Test 3: Multi Row Query

    http://localhost:8000/queries?q=20

### Test 4: Fortunes (Template rendering)

    http://localhost:8000/fortunes

### Test 5: Update Query

    http://localhost:8000/updates?q=20

### Test 6: Plaintext

    http://localhost:8000/plaintext