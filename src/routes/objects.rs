use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::Deserialize;

use crate::startup::AppState;

#[derive(Deserialize)]
pub struct FetchObjectsParams {
    said: String,
}

pub async fn fetch_objects(
    app_state: web::Data<AppState>,
    query_params: web::Query<FetchObjectsParams>,
) -> HttpResponse {
    let saids: Vec<String> = query_params
        .said
        .split(',')
        .map(|s| s.to_string())
        .collect();

    let result = match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_oca_objects(saids)
    {
        Ok(objects) => {
            serde_json::json!({
                "success": true,
                "objects": objects,
            })
        }
        Err(e) => {
            println!("{:?}", e);
            serde_json::json!({
                "success": false,
                "errors": e
            })
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}
