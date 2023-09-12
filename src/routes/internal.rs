use actix_web::{http::header::ContentType, web, HttpResponse};
use oca_rs::{
    data_storage::DataStorage, repositories::SQLiteConfig, EncodeBundle,
};

#[derive(serde::Deserialize)]
pub struct FetchAllOCABundleParams {
    page: Option<usize>,
}

pub async fn fetch_all_oca_bundle(
    db: web::Data<Box<dyn DataStorage>>,
    cache_storage: web::Data<SQLiteConfig>,
    query_params: web::Query<FetchAllOCABundleParams>,
) -> HttpResponse {
    let oca_facade = oca_rs::Facade::new(
        db.get_ref().clone(),
        cache_storage.get_ref().clone(),
    );
    let page = query_params.page.unwrap_or(1);
    let result = match oca_facade.fetch_all_oca_bundle(100, page) {
        Ok(all_oca_bundles) => {
            serde_json::json!({
                "success": true,
                "r":
                    all_oca_bundles.records.iter().map(|oca_bundle| {
                        serde_json::from_str(
                            &String::from_utf8(
                                oca_bundle.encode().unwrap()
                            ).unwrap()
                        ).unwrap()
                    }).collect::<Vec<serde_json::Value>>(),
                "m": all_oca_bundles.metadata,
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

#[derive(serde::Deserialize)]
pub struct FetchAllCaptureBaseParams {
    page: Option<usize>,
}

pub async fn fetch_all_capture_base(
    db: web::Data<Box<dyn DataStorage>>,
    cache_storage: web::Data<SQLiteConfig>,
    query_params: web::Query<FetchAllCaptureBaseParams>,
) -> HttpResponse {
    let oca_facade = oca_rs::Facade::new(
        db.get_ref().clone(),
        cache_storage.get_ref().clone(),
    );
    let page = query_params.page.unwrap_or(1);
    let result = match oca_facade.fetch_all_capture_base(100, page) {
        Ok(all_capture_bases) => {
            serde_json::json!({
                "success": true,
                "r": all_capture_bases.records,
                "m": all_capture_bases.metadata,
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
