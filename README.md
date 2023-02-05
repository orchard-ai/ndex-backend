# Mentat Backend
Mentat backend webserver in Rust.

Currently working feature:
- Authentication with Notion:
    - Insert row into Notion Database
    - Can return all objects within Notion Database
- Create/Retrieve/Delete Typesense schemas from the webserver endpoint
## Todos
- Dynamically query all objects within Notion Database for results with more than 100 objects.
- Implement OAuth2 login to get the Notion credentials, instead of using .env file.

# How to run
1. Have Rust toolchain installed
2. Create a .env file with NOTION_DATABASE_ID and NOTION_SECRET set.
3. `cargo run`

Alternatively for development, you can install cargo-watch and have the webserver recompile upon saving any file:

`cargo watch -x run -B 1`

