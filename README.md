# rsyesql

[![crates.io](https://img.shields.io/crates/v/rsyesql.svg)](https://crates.io/crates/rsyesql)
[![docs.rs](https://docs.rs/rsyesql/badge.svg)](https://docs.rs/rsyesql)
[![ci](https://github.com/fanatid/rsyesql/workflows/ci/badge.svg)](https://github.com/fanatid/rsyesql/actions?query=workflow%3Aci)

Inspired by [Yesql](https://github.com/krisajenkins/yesql), see [rational section](https://github.com/krisajenkins/yesql#rationale) there for more info.

### Usage

`queries.sql`:

```sql
-- name: select
SELECT * FROM users;

-- name: delete
DELETE FROM users WHERE id = $1;
```

In `Rust` code:
```rust
let queries = rsyesql::parse(include_str!("./queries.sql"));
println!("{}", queries.get("select").unwrap()); // SELECT * FROM users;
println!("{}", queries.get("delete").unwrap()); // DELETE FROM users WHERE id = $1;
```

### LICENSE [MIT](LICENSE)
