# ndex Backend
ndex backend webserver in Rust using the Axum web framework.

# Dev Environment Setup (Mac/Linux. For Windows, install Ubuntu)
Run the following commands in your terminal:
- Install [Rust](https://www.rust-lang.org/tools/install), run this command and follow the instructions that appear in the terminal: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install Docker
- Pull Typesense [image](https://hub.docker.com/r/typesense/typesense), this is the Search API we use: `docker pull typesense/typesense:0.24.0`
- Run Typesense (Keep open): `docker run -p 8108:8108 -v/tmp/data:/data typesense/typesense:0.24.0 --data-dir /data --api-key=xyz`
- Install the Rust VSCode extension to prevent having to recompile upon code changes: install Cargo Watch then run `cargo watch -x run -B 1`
- Create a `.env` file (ask Pan for the dev notion secret):
    ```
    NOTION_SECRET=
    TYPESENSE_SECRET=xyz
    ```
- Run backend Rust server: `cargo run`
- Access endpoints through http://www.localhost:3001
- Finally, to connect with frontend, run the frontend, go to the settings page on the UI and click on "Connect Backend". It may take a minute or two.

# Todos
- Implement OAuth2 login to get the Notion credentials, instead of using .env file.
- Dynamically define the schema name/properties
- Use TOML config file instead of .env

# Currently working feature:
- Authentication with Notion:
    - Insert row into Notion Database
    - Can return all objects within Notion Database
- Create/Retrieve/Delete Typesense schemas from the webserver endpoint
- Dynamically query all objects within Notion Database for results with more than 100 objects:
    - Gets the pages and database, then gets all the blocks that are contained within the pages and databases
- Parsing of notion data and saving it as JsonLines format
- Sends JsonLine data to Typesense server for indexing

Basically MVP endpoints for front-end dev should be complete

# Endpoints
- GET "/" -> Hello World
- GET "/notion/search_notion" -> Retrieve all Notion objects in a workspace, parses them, and saves them in a JSONL file locally
- GET "/typesense/create_typesense_schema" -> Creates a "documents" schema on the local Typesense server
- GET "/typesense/delete_typesense_schema" -> Deletes "documents" schema on the local Typesense server
- GET "/typesense/retrieve_typesense_schema" -> Retrieves all schemas on the local Typesense server
- GET "/typesense/batch_index" -> Sends parsed Notion data to Typesense for indexing
