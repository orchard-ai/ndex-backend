# Mentat Backend
Mentat backend webserver in Rust.

Currently working feature:
- Authentication with Notion:
    - Insert row into Notion Database
    - Can return all objects within Notion Database
- Create/Retrieve/Delete Typesense schemas from the webserver endpoint
- Dynamically query all objects within Notion Database for results with more than 100 objects:
    - Gets the pages and database, then gets all the blocks that are contained within the pages and databases
## Todos
- Implement OAuth2 login to get the Notion credentials, instead of using .env file.

# How to run
1. Have Rust toolchain installed
2. Create a .env file with NOTION_DATABASE_ID and NOTION_SECRET set.
3. `cargo run`
4. Access endpoints on your localhost:3001

Alternatively for development, you can install cargo-watch and have the webserver recompile upon saving any file:

`cargo watch -x run -B 1`

# Endpoints
- GET "/" -> Hello World
- POST "/notion/create_notion_row" -> Insert row into Notion DB
- GET "/notion/search_notion" -> Retrieve all Notion objects in a workspace
- GET "/typesense/create_typesense_schema" -> Creates a "documents" schema on the local Typesense server
- GET "/typesense/delete_typesense_schema" -> Deletes "documents" schema on the local Typesense server
- GET "/typesense/retrieve_typesense_schema" -> Retrieves all schemas on the local Typesense server