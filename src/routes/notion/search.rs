use reqwest::Client;
use serde_json::json;

use super::SearchResponse;

async fn search(
    client: &Client,
    cursor: Option<String>,
    query: String,
) -> Result<SearchResponse, Box<dyn std::error::Error + Send + Sync>> {
    let search_query = match &cursor {
        Some(uuid) => json!({
            "query": query,
            "start_cursor": uuid
        }),
        None => {
            json!( {
                "query": "".to_string(),
            })
        }
    };

    let request = client
        .post("https://api.notion.com/v1/search")
        .json(&search_query);

    let response = request.send().await?;
    let response_body: SearchResponse = response.json().await?;
    Ok(response_body)
}
