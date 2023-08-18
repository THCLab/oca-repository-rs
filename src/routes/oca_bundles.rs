use oca_rs::data_storage::DataStorage;
use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

pub async fn add_oca_file(
    db: web::Data<Box<dyn DataStorage>>,
    item: web::Bytes,
    _req: HttpRequest,
) -> HttpResponse {
    let ocafile = String::from_utf8(item.to_vec()).unwrap();

    let oca_facade = oca_rs::Facade::new(db.get_ref().clone());
    let result = match oca_facade.build_from_ocafile(ocafile) {
        Ok(oca_bundle) => {
            serde_json::json!({
                "success": true,
                "said": oca_bundle.said.unwrap(),
            })
        },
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

pub async fn get_oca_bundle(
  db: web::Data<Box<dyn DataStorage>>,
  req: HttpRequest,
) -> HttpResponse {
    let said = req.match_info().get("said").unwrap().to_string();

    let oca_facade = oca_rs::Facade::new(db.get_ref().clone());
    let result = match oca_facade.get_oca_bundle(said) {
        Ok(oca_bundle) => {
            serde_json::to_value(&oca_bundle).unwrap()
        },
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

#[derive(Deserialize)]
pub struct OCAFileHistoryQueryParams {
    extend: Option<bool>,
}

pub async fn get_oca_file_history(
    db: web::Data<Box<dyn DataStorage>>,
    req: HttpRequest,
    query_params: web::Query<OCAFileHistoryQueryParams>,
) -> HttpResponse {
    #[derive(Serialize)]
    struct Item {
        from: Option<serde_value::Value>,
        operation: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        oca_bundle: Option<serde_json::Value>,
    }

    let said = req.match_info().get("said").unwrap().to_string();

    let oca_facade = oca_rs::Facade::new(db.get_ref().clone());
    let result = match oca_facade.get_oca_bundle_steps(said) {
        Ok(oca_build_steps) => {
            serde_json::to_value(
                &oca_build_steps.iter().map(|s| {
                    serde_json::to_value(
                        &Item {
                            from: s.parent_said.clone().map(|said| serde_value::Value::String(said.to_string())),
                            operation: serde_json::to_value(&s.command).unwrap(),
                            oca_bundle: if query_params.extend.unwrap_or(false) {
                                Some(serde_json::to_value(&s.result).unwrap())
                            } else {
                                None
                            },
                        }
                    ).unwrap()
                }).collect::<Vec<serde_json::Value>>()
            ).unwrap()
        },
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

pub async fn get_oca_file(
    db: web::Data<Box<dyn DataStorage>>,
    req: HttpRequest,
) -> HttpResponse {
    let said = req.match_info().get("said").unwrap().to_string();

    let oca_facade = oca_rs::Facade::new(db.get_ref().clone());
    match oca_facade.get_oca_bundle_ocafile(said) {
        Ok(ocafile) => {

            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(ocafile)
        },
        Err(errors) => {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(
                    serde_json::to_string(
                        &serde_json::json!({
                            "success": false,
                            "errors": errors,
                        })
                    ).unwrap()
                )
        }
    }

}
