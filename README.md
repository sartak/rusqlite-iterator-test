```
$ cargo build
error[E0515]: cannot return value referencing local variable `stmt`
  --> src/main.rs:29:5
   |
22 |       let results = stmt.query_map([], |row| {
   |  ___________________-
23 | |         Ok(Person {
24 | |             id: row.get(0)?,
25 | |             name: row.get(1)?,
26 | |         })
27 | |     })?;
   | |______- `stmt` is borrowed here
28 |
29 | /     Ok(PersonIterator {
30 | |         results: Box::new(results),
31 | |     })
   | |______^ returns a value referencing data owned by the current function

For more information about this error, try `rustc --explain E0515`.
error: could not compile `rusqlite-iterator-test` due to previous error
```
