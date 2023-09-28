use actix_web::{http::header::ContentType, web, HttpResponse};
use oca_rs::{data_storage::DataStorage, repositories::SQLiteConfig};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FetchObjectsParams {
    said: String,
}

pub async fn fetch_objects(
    db: web::Data<Box<dyn DataStorage>>,
    cache_storage: web::Data<SQLiteConfig>,
    query_params: web::Query<FetchObjectsParams>,
) -> HttpResponse {
    let saids: Vec<String> = query_params
        .said
        .split(',')
        .map(|s| s.to_string())
        .collect();

    let oca_facade = oca_rs::Facade::new(
        db.get_ref().clone(),
        cache_storage.get_ref().clone(),
    );

    let result = match oca_facade.get_oca_objects(saids) {
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
