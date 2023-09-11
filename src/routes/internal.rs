use actix_web::{http::header::ContentType, web, HttpResponse};
use oca_rs::{
    data_storage::DataStorage, repositories::SQLiteConfig, EncodeBundle,
};

pub async fn fetch_all_oca_bundle(
    db: web::Data<Box<dyn DataStorage>>,
    cache_storage: web::Data<SQLiteConfig>,
) -> HttpResponse {
    let oca_facade = oca_rs::Facade::new(
        db.get_ref().clone(),
        cache_storage.get_ref().clone(),
    );
    let result = match oca_facade.fetch_all_oca_bundle(500) {
        Ok(oca_bundles) => {
            serde_json::json!({
                "success": true,
                "results":
                    oca_bundles.iter().map(|oca_bundle| {
                        serde_json::from_str(
                            &String::from_utf8(
                                oca_bundle.encode().unwrap()
                            ).unwrap()
                        ).unwrap()
                    }).collect::<Vec<serde_json::Value>>(),
            })
        }
        Err(errors) => {
            serde_json::json!({
                "success": false,
                "errors": errors,
            })
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

pub async fn fetch_all_capture_base(
    db: web::Data<Box<dyn DataStorage>>,
    cache_storage: web::Data<SQLiteConfig>,
) -> HttpResponse {
    let oca_facade = oca_rs::Facade::new(
        db.get_ref().clone(),
        cache_storage.get_ref().clone(),
    );
    let result = match oca_facade.fetch_all_capture_base(500) {
        Ok(capture_bases) => {
            serde_json::json!({
                "success": true,
                "results": capture_bases,
            })
        }
        Err(errors) => {
            serde_json::json!({
                "success": false,
                "errors": errors,
            })
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}
