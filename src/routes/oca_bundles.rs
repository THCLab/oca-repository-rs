use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
// use cached::IOCached;
use crate::startup::AppState;
use oca_rs::{facade::bundle::BundleElement, EncodeBundle, HashFunctionCode, SerializationFormats};
use said::SelfAddressingIdentifier;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub async fn add_oca_file(
    app_state: web::Data<AppState>,
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
    // println!("Got ocafile: {}", &ocafile);
    if ocafile.is_empty() {
        let error = "OCA File can't be empty";
        return HttpResponse::UnprocessableEntity()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&vec![error]).unwrap());
    }
    let cached = app_state.cache.get(&ocafile);
    let result = match cached {
        Ok(Some(cached_said)) => {
            println!("Getting from cache: {}", &cached_said);
            serde_json::json!({
                "success": true,
                "said": cached_said,
            })
        }
        Ok(None) => {
            println!("New ocafile. Need to rebuild");
            let built_result = {
               app_state
                    .facade
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .build_from_ocafile(ocafile.clone())
            };
            
            match built_result
            {
                Ok(oca_bundle) => match oca_bundle {
                    BundleElement::Mechanics(mechanics) => {
                        let said = mechanics.said.clone();
                        // println!("Rebuilt successfully: {:?}", &said);
                        // println!("Saving to cache: {}", &said.as_ref().unwrap());
                        if let Err(e) = app_state.cache.insert(&ocafile, mechanics.said.unwrap()) {
                            return HttpResponse::InternalServerError()
                                .content_type(ContentType::json())
                                .body(e.to_string());
                        };
                        serde_json::json!({
                            "success": true,
                            "said": said.unwrap(),
                        })
                    }
                    _ => serde_json::json!({
                        "success": false,
                    }),
                },
                Err(errors) => {
                    serde_json::json!({
                        "success": false,
                        "errors": errors,
                    })
                }
            }
        }
        Err(e) => {
            println!("Error: {}", &e);
            return HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(e.to_string())
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
    app_state: web::Data<AppState>,
    header_language: web::Header<actix_web::http::header::AcceptLanguage>,
    query_params: web::Query<SearchParams>,
) -> HttpResponse {
    let limit = 10;
    let code = HashFunctionCode::Blake3_256;
    let format = SerializationFormats::JSON;

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

    let search_result = app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .search_oca_bundle(
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
                            r.oca_bundle.encode(&code, &format).unwrap()
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

#[derive(Deserialize)]
pub struct OCABundleQueryParams {
    w: Option<bool>, // with_dependencies
}

pub async fn get_oca_bundle(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    query_params: web::Query<OCABundleQueryParams>,
) -> HttpResponse {
    let said_str = req.match_info().get("said").unwrap().to_string();
    let code = HashFunctionCode::Blake3_256;
    let format = SerializationFormats::JSON;
    let said = match SelfAddressingIdentifier::from_str(&said_str) {
        Ok(said) => said,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&vec![format!("Invalid SAID: {}", e)]).unwrap())
        }
    };

    let with_dependencies = query_params.w.unwrap_or(false);

    let result = match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_oca_bundle(said, with_dependencies)
    {
        Ok(oca_bundle) => serde_json::to_string(&serde_json::json!({
            "bundle":
                serde_json::from_str::<serde_json::Value>(
                    &String::from_utf8(
                        oca_bundle.bundle.encode(&code, &format).unwrap()
                    ).unwrap()
                ).unwrap(),
            "dependencies": oca_bundle.dependencies.iter().map(|d| {
                serde_json::from_str::<serde_json::Value>(
                    &String::from_utf8(
                        d.encode(&code, &format).unwrap()
                    ).unwrap()
                ).unwrap()
            }).collect::<Vec<serde_json::Value>>(),
        }))
        .expect("Failed to serialize oca_bundle"),
        Err(errors) => serde_json::to_string(&serde_json::json!({
            "success": false,
            "errors": errors,
        }))
        .expect("Failed to serialize errors"),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result)
}

#[derive(Deserialize)]
pub struct OCAFileHistoryQueryParams {
    extend: Option<bool>,
}

pub async fn get_oca_file_history(
    app_state: web::Data<AppState>,
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

    let said_str = req.match_info().get("said").unwrap().to_string();
    let said = match SelfAddressingIdentifier::from_str(&said_str) {
        Ok(said) => said,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&vec![format!("Invalid SAID: {}", e)]).unwrap())
        }
    };

    let result = match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_oca_bundle_steps(said)
    {
        Ok(oca_build_steps) => serde_json::to_value(
            oca_build_steps
                .iter()
                .map(|s| {
                    serde_json::to_value(&Item {
                        from: s
                            .parent_said
                            .clone()
                            .map(|said| serde_value::Value::String(said.to_string())),
                        operation: serde_json::to_value(&s.command).unwrap(),
                        oca_bundle: if query_params.extend.unwrap_or(false) {
                            Some(serde_json::to_value(&s.result).unwrap())
                        } else {
                            None
                        },
                    })
                    .unwrap()
                })
                .collect::<Vec<serde_json::Value>>(),
        )
        .unwrap(),
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

pub async fn get_oca_file(app_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let said_str = req.match_info().get("said").unwrap().to_string();
    let said = match SelfAddressingIdentifier::from_str(&said_str) {
        Ok(said) => said,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&vec![format!("Invalid SAID: {}", e)]).unwrap())
        }
    };

    match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_oca_bundle_ocafile(said, true)
    {
        Ok(ocafile) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(ocafile),
        Err(errors) => HttpResponse::Ok().content_type(ContentType::json()).body(
            serde_json::to_string(&serde_json::json!({
                "success": false,
                "errors": errors,
            }))
            .unwrap(),
        ),
    }
}

#[cfg(feature = "data_entries_xls")]
pub async fn get_oca_data_entry(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> actix_web::Result<actix_files::NamedFile> {
    let configuration = crate::configuration::get_configuration()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let data_entries_path = configuration
        .application
        .data_entries_path
        .unwrap_or("".to_string());
    let said_str = req.match_info().get("said").unwrap().to_string();
    let said = SelfAddressingIdentifier::from_str(&said_str)
        .map_err(|e| actix_web::error::ErrorUnprocessableEntity(format!("Invalid SAID: {}", e)))?;

    let oca_bundle = app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_oca_bundle(said, false)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.first().unwrap().clone()))?;
    let oca_bundle_list = vec![oca_bundle.bundle.clone()];
    let _ = oca_parser_xls::xls_parser::data_entry::generate(
        oca_bundle_list.as_slice(),
        format!(
            "{}/{}",
            data_entries_path.clone(),
            oca_bundle.bundle.said.clone().unwrap()
        ),
    );
    Ok(actix_files::NamedFile::open(format!(
        "{}/{}-data_entry.xlsx",
        data_entries_path,
        oca_bundle.bundle.said.clone().unwrap()
    ))?)
}
