use std::sync::Mutex;

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};

pub async fn fetch_relations(
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    req: HttpRequest,
) -> HttpResponse {
    let said = req.match_info().get("said").unwrap().to_string();

    let result = match oca_facade.lock().unwrap().explore(said) {
        Some(relationship) => {
            serde_json::json!({
                "success": true,
                "object_type": relationship.base_object.object_type,
                "relatives": relationship.relations.iter().map(|r| {
                    serde_json::to_value(r).unwrap()
                }).collect::<Vec<_>>(),
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
