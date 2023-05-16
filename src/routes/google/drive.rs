use crate::{
    models::gdrive::{File, GDriveResponse},
    routes::typesense::{Product, RowType, TypesenseInsert},
};

use axum::response::IntoResponse;
use axum::Json;
use chrono::DateTime;
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
    let response: Vec<File> = retrieve_file_list(&client)
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

async fn retrieve_file_list(client: &Client) -> Result<Vec<File>, Error> {
    info!("Retrieving messages list");
    let mut cursor: Option<String> = None;
    let mut file_list: Vec<File> = vec![];
    loop {
        let url: String;
        let params =
            "fields=kind,incompleteSearch,nextPageToken,files(id,name,mimeType,createdTime,modifiedTime,webViewLink,owners)";
        if let Some(page_id) = cursor {
            url = format!(
                "https://www.googleapis.com/drive/v3/files?{}&pageToken={}",
                params, page_id,
            )
        } else {
            url = format!("https://www.googleapis.com/drive/v3/files?{}", params)
        }
        dbg!(&url);
        let text = client.get(&url).send().await?.text().await?;

        let response: Result<GDriveResponse, serde_json::Error> = from_str(&text);

        match response {
            Ok(parsed) => {
                dbg!(parse_file(parsed.files[0].clone()));
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
    Ok(file_list)
}

fn parse_file(file: File) -> TypesenseInsert {
    let email = "placeholder".to_string();
    let id = file.id;
    let contents = file.mime_type;
    let title = file.name;
    let url = file.web_view_link;
    let added_by = Some(
        file.owners
            .iter()
            .map(|owner| owner.email_address.to_owned())
            .collect::<Vec<String>>()
            .join(", "),
    );
    let platform = Product::GDrive;
    let type_field = RowType::File;
    let last_edited_time = DateTime::parse_from_rfc3339(&file.modified_time)
        .unwrap()
        .timestamp();
    let created_time = DateTime::parse_from_rfc3339(&file.created_time)
        .unwrap()
        .timestamp();
    TypesenseInsert {
        account_email: email,
        id,
        title,
        contents,
        url,
        added_by,
        platform,
        type_field,
        last_edited_time,
        created_time,
    }
}
