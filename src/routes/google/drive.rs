use crate::models::gdrive::{File, GDriveResponse};

use axum::response::IntoResponse;
use axum::Json;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::{Client, Error};
use serde_json::{from_str, Value};
use tracing::info;

pub async fn gdrive_request(headers: HeaderMap) -> Result<impl IntoResponse, String> {
    let auth_header = headers.get("Authorization").unwrap();
    let access_token = auth_header.to_str().unwrap().replace("Bearer ", "");
    let mut h = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    h.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    let client = Client::builder().default_headers(h).build().unwrap();
    let response: Vec<Value> = retrieve_file_list(&client)
        .await
        .map_err(|e| e.to_string())?;
    Ok((StatusCode::OK, Json(response)))
}

// pub async fn index_gdrive_handler(
//     State(jwt_secret): State<String>,
//     State(pool): State<Pool<Postgres>>,
//     State(typesense_secret): State<TypesenseSecret>,
//     headers: HeaderMap,
//     Json(payload): Json<IndexGMailRequest>,
// ) -> impl IntoResponse {
//     let auth_header = headers.get("Authorization").unwrap();
//     let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
//     if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
//         let user_id = claims.sub;
//         let email = payload.email;
//         let access_token = get_access_token(&pool, &user_id, &email, Platform::Google).await?;
//         index(&access_token, &user_id, &email)
//             .await
//             .map_err(|e| UserError::InternalServerError(e.to_string()))?;
//         match batch_index(&typesense_secret.0, &user_id, Product::GMail).await {
//             Ok(_) => {
//                 return Ok((
//                     StatusCode::OK,
//                     Json(json!({"message": "indexing complete".to_string()})),
//                 ))
//             }
//             Err(e) => {
//                 return Err(UserError::InternalServerError(e.to_string()));
//             }
//         }
//     }
//     Err(UserError::Unauthorized("Wrong token".to_string()))
// }

// async fn index(access_token: &str, user_id: &str, email: &str) -> Result<String, String> {
//     let filepath = format!("google_calendar_events_{}.jsonl", user_id);
//     File::create(&filepath).map_err(|e| e.to_string())?;

//     let mut headers = HeaderMap::new();
//     let bearer = format!("Bearer {}", access_token);
//     headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
//     let client = Client::builder().default_headers(headers).build().unwrap();
//     Ok("Indexed".to_string())
// }

async fn retrieve_file_list(client: &Client) -> Result<Vec<Value>, Error> {
    info!("Retrieving messages list");
    let mut cursor: Option<String> = None;
    let mut file_list: Vec<Value> = vec![];
    loop {
        let url: String;
        if let Some(page_id) = cursor {
            url = format!(
                "https://www.googleapis.com/drive/v3/files?pageToken={}&fields=kind,incompleteSearch,nextPageToken,files(id,name,mimeType,size,createdTime)",
                page_id
            )
        } else {
            url = "https://www.googleapis.com/drive/v3/files?fields=kind,incompleteSearch,nextPageToken,files(id,name,mimeType,size,createdTime)".to_string();
        }
        dbg!(&url);
        let text = client.get(&url).send().await?.text().await?;

        let response: Result<GDriveResponse, serde_json::Error> = from_str(&text);

        match response {
            Ok(parsed) => {
                file_list.extend(parsed.files);
                if file_list.len() > 500 {
                    break;
                }
                if let Some(next_page) = parsed.next_page_token {
                    cursor = Some(next_page);
                } else {
                    break;
                }
            }
            Err(e) => {
                println!("Failed to parse response: {:?}", e);
                println!("Raw response: {}", from_str::<Value>(&text).unwrap());
                dbg!(file_list.len());
                break;
            }
        }
    }
    info!("Finished retrieving messages list");
    // let file = &file_list.last().unwrap().id;
    // let file_object = get_file_object(file, client).await?;
    // dbg!(file_object);
    Ok(file_list)
}

async fn get_file_object(file_id: &str, client: &Client) -> Result<serde_json::Value, Error> {
    let url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?fields=id,name,mimeType,size,createdTime",
        file_id
    );
    let response: serde_json::Value = client.get(url).send().await?.json().await?;
    Ok(response)
}
