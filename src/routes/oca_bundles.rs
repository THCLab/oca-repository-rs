use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use oca_rs::EncodeBundle;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Mutex};

pub async fn add_oca_file(
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    item: web::Bytes,
    _req: HttpRequest,
) -> HttpResponse {
    let ocafile = match String::from_utf8(item.to_vec()) {
        Ok(parsed) => parsed,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&vec![e.to_string()]).unwrap())
        }
    };

    if ocafile.is_empty() {
        let error = "OCA File can't be empty";
        return HttpResponse::UnprocessableEntity()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&vec![error]).unwrap())
    }

    let result = match oca_facade.lock().unwrap().build_from_ocafile(ocafile) {
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

#[derive(Deserialize)]
pub struct SearchParams {
    q: Option<String>,
    lang: Option<isolang::Language>,
    page: Option<usize>,
}

pub async fn search(
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    header_language: web::Header<actix_web::http::header::AcceptLanguage>,
    query_params: web::Query<SearchParams>,
) -> HttpResponse {
    let limit = 10;

    let language = if query_params.lang.is_none() {
        isolang::Language::from_str(
            header_language
                .preference()
                .to_string()
                .split('-')
                .collect::<Vec<&str>>()
                .first()
                .unwrap(),
        )
        .ok()
    } else {
        query_params.lang
    };

    let search_result = oca_facade.lock().unwrap().search_oca_bundle(
        language,
        query_params.q.clone().unwrap_or("".to_string()),
        limit,
        query_params.page.unwrap_or(1),
    );

    let result = serde_json::json!({
        "r": search_result.records.iter().map(|r| {
            serde_json::json!({
                "oca_bundle":
                    serde_json::from_str::<serde_json::Value>(
                        &String::from_utf8(
                            r.oca_bundle.encode().unwrap()
                        ).unwrap()
                    ).unwrap(),
                "metadata": r.metadata,
            })
        }).collect::<Vec<serde_json::Value>>(),
        "m": search_result.metadata,
    });
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

pub async fn get_oca_bundle(
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    req: HttpRequest,
) -> HttpResponse {
    let said = req.match_info().get("said").unwrap().to_string();

    let result = match oca_facade.lock().unwrap().get_oca_bundle(said) {
        Ok(oca_bundle) => {
            serde_json::from_str(
                &String::from_utf8(oca_bundle.encode().unwrap()).unwrap()
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

#[derive(Deserialize)]
pub struct OCAFileHistoryQueryParams {
    extend: Option<bool>,
}

pub async fn get_oca_file_history(
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
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

    let result = match oca_facade.lock().unwrap().get_oca_bundle_steps(said) {
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
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    req: HttpRequest,
) -> HttpResponse {
    let said = req.match_info().get("said").unwrap().to_string();

    match oca_facade.lock().unwrap().get_oca_bundle_ocafile(said) {
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

#[cfg(feature = "data_entries_xls")]
pub async fn get_oca_data_entry(
    oca_facade: web::Data<Mutex<oca_rs::Facade>>,
    req: HttpRequest,
) -> actix_web::Result<actix_files::NamedFile> {
    let configuration = crate::configuration::get_configuration()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let data_entries_path = configuration
        .application
        .data_entries_path
        .unwrap_or("".to_string());
    let said = req.match_info().get("said").unwrap().to_string();

    let oca_bundle = oca_facade.lock().unwrap().get_oca_bundle(said).map_err(|e| {
        actix_web::error::ErrorInternalServerError(e.first().unwrap().clone())
    })?;
    let oca_bundle_list = vec![oca_bundle.clone()];
    let _ = oca_parser_xls::xls_parser::data_entry::generate(
        &oca_bundle_list,
        format!(
            "{}/{}",
            data_entries_path.clone(),
            oca_bundle.said.clone().unwrap()
        ),
    );
    Ok(actix_files::NamedFile::open(format!(
        "{}/{}-data_entry.xlsx",
        data_entries_path,
        oca_bundle.said.clone().unwrap()
    ))?)
}
