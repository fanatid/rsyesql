# rsyesql

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
