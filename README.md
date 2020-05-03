# book-api
A sample API for books on rust-lang

## Database setup locally with Diesel
1. Set up the database with
`$ diesel setup`
2. Generate `up.sql` and `down.sql` by
`$ diesel migration generate create_books`
3. Edit the `up.sql` and `down.sql` files with DMLs and run the migration 
`$ diesel migration run`
4. It is good practice to run rollback immediately to check rollback
`$ diesel migration redo`
5. Add table macro to `schema.rs`
`$ diesel print-schema > src/schema.rs`