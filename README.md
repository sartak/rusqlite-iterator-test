```
$ cargo build
error[E0515]: cannot return value referencing local variable `stmt`
  --> src/main.rs:17:5
   |
10 |       let results = stmt.query_map([], |row| {
   |  ___________________-
11 | |         Ok(Person {
12 | |             id: row.get(0)?,
13 | |             name: row.get(1)?,
14 | |         })
15 | |     })?;
   | |______- `stmt` is borrowed here
16 |
17 |       Ok(Box::new(results))
   |       ^^^^^^^^^^^^^^^^^^^^^ returns a value referencing data owned by the current function

error[E0515]: cannot return value referencing function parameter `conn`
  --> src/main.rs:17:5
   |
9  |     let mut stmt = conn.prepare("SELECT rowid, name FROM people")?;
   |                    ---------------------------------------------- `conn` is borrowed here
...
17 |     Ok(Box::new(results))
   |     ^^^^^^^^^^^^^^^^^^^^^ returns a value referencing data owned by the current function

For more information about this error, try `rustc --explain E0515`.
error: could not compile `rusqlite-iterator-test` due to 2 previous errors
```
