use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use oca_rs::{data_storage::DataStorage, repositories::SQLiteConfig};

pub async fn fetch_relations(
    db: web::Data<Box<dyn DataStorage>>,
    cache_storage: web::Data<SQLiteConfig>,
    req: HttpRequest,
) -> HttpResponse {
    let said = req.match_info().get("said").unwrap().to_string();

    let oca_facade = oca_rs::Facade::new(
        db.get_ref().clone(),
        cache_storage.get_ref().clone(),
    );

    let result = match oca_facade.explore(said) {
        Some(relationship) => {
            serde_json::json!({
                "success": true,
                "object_type": relationship.base_object.object_type,
                "relatives": relationship.relations.iter().map(|r| {
                    serde_json::json!({
                        "said": r.said,
                        "object_type": r.object_type,
                    })
                }).collect::<Vec<serde_json::Value>>(),

            })
        }
        None => {
            serde_json::json!({
                "success": false
            })
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}
