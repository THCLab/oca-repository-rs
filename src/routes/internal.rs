use actix_web::{http::header::ContentType, web, HttpResponse};

use crate::startup::AppState;

#[derive(serde::Deserialize)]
pub struct FetchAllOCABundleParams {
    page: Option<usize>,
    limit: Option<usize>,
}

pub async fn fetch_all_oca_bundle(
    app_state: web::Data<AppState>,
    query_params: web::Query<FetchAllOCABundleParams>,
) -> HttpResponse {
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(100);

    let result = match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .fetch_all_oca_bundle(limit, page)
    {
        Ok(all_oca_bundles) => {
            serde_json::json!({
                "success": true,
                "r":
                    all_oca_bundles.records,
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
    limit: Option<usize>,
}

pub async fn fetch_all_capture_base(
    app_state: web::Data<AppState>,
    query_params: web::Query<FetchAllCaptureBaseParams>,
) -> HttpResponse {
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(100);
    let result = match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .fetch_all_capture_base(limit, page)
    {
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
