# Mentat Backend
Mentat backend webserver in Rust.

Currently working feature:
- Authentication with Notion:
    - Insert row into Notion Database

# How to run
1. Have Rust toolchain installed
2. Create a .env file with NOTION_DATABASE_ID and NOTION_SECRET set.
3. `cargo run`

Alternatively for development, you can install cargo-watch and have the webserver recompile upon saving any file:

`cargo watch -x run -B 1`