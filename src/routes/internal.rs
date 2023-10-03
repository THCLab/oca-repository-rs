use std::sync::Mutex;

use actix_web::{http::header::ContentType, web, HttpResponse};
use oca_rs::EncodeBundle;

#[derive(serde::Deserialize)]
pub struct FetchAllOCABundleParams {
    page: Option<usize>,
}

pub async fn fetch_all_oca_bundle(
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    query_params: web::Query<FetchAllOCABundleParams>,
) -> HttpResponse {
    let page = query_params.page.unwrap_or(1);
    let result = match oca_facade.lock().unwrap().fetch_all_oca_bundle(100, page) {
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
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    query_params: web::Query<FetchAllCaptureBaseParams>,
) -> HttpResponse {
    let page = query_params.page.unwrap_or(1);
    let result = match oca_facade.lock().unwrap().fetch_all_capture_base(100, page) {
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
