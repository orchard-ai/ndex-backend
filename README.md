# Mentat Backend
Mentat backend webserver in Rust.

Currently working feature:
- Authentication with Notion:
    - Insert row into Notion Database
    - Can return all objects within Notion Database
- Create/Retrieve/Delete Typesense schemas from the webserver endpoint
- Dynamically query all objects within Notion Database for results with more than 100 objects:
    - Gets the pages and database, then gets all the blocks that are contained within the pages and databases
- Parsing of notion data and saving it as JsonLines format
- Sends JsonLine data to Typesense server for indexing

Basically MVP endpoints for front-end dev should be complete

## Todos
- Implement OAuth2 login to get the Notion credentials, instead of using .env file.
- Dynamically define the schema name/properties
- Use TOML config file instead of .env

# Dev Environment Setup
- Install and run Typesense locally (I recommend pulling their Docker Image and running it):
    - Make sure its running on port 8108
- Install the Rust toolchain

# How to run
1. Create a .env file and NOTION_SECRET and TYPESENSE_SECRET set.
2. `cargo run`
3. Access endpoints on your localhost:3001

Alternatively for development, you can install cargo-watch and have the webserver recompile upon saving any file:

`cargo watch -x run -B 1`

# Endpoints
- GET "/" -> Hello World
- GET "/notion/search_notion" -> Retrieve all Notion objects in a workspace, parses them, and saves them in a JSONL file locally
- GET "/typesense/create_typesense_schema" -> Creates a "documents" schema on the local Typesense server
- GET "/typesense/delete_typesense_schema" -> Deletes "documents" schema on the local Typesense server
- GET "/typesense/retrieve_typesense_schema" -> Retrieves all schemas on the local Typesense server
- GET "/typesense/batch_index" -> Sends parsed Notion data to Typesense for indexing