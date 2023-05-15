use crate::models::gdrive::GDriveResponse;

use axum::response::IntoResponse;
use axum::Json;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::{Client, Error};
use tracing::info;

pub async fn gdrive_request(headers: HeaderMap) -> Result<impl IntoResponse, String> {
    let auth_header = headers.get("Authorization").unwrap();
    let access_token = auth_header.to_str().unwrap().replace("Bearer ", "");
    let mut h = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    h.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    let client = Client::builder().default_headers(h).build().unwrap();
    let response: GDriveResponse = retrieve_gdrive(&client).await.map_err(|e| e.to_string())?;
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

async fn retrieve_gdrive(client: &Client) -> Result<GDriveResponse, Error> {
    info!("Retrieving messages list");
    dbg!(client);
    let next_url = "https://www.googleapis.com/drive/v3/files".to_string();
    let response = client.get(next_url).send().await?;
    let files: GDriveResponse = response.json().await?;
    info!("Finished retrieving messages list");
    Ok(files)
}
