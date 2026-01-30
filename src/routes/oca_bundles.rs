use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use oca_sdk_rs::OCABundle;
use tracing::info;
// use cached::IOCached;
use crate::startup::AppState;
use said::SelfAddressingIdentifier;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;

pub async fn add_oca_file(
    app_state: web::Data<AppState>,
    item: web::Bytes,
    _req: HttpRequest,
) -> HttpResponse {
    info!("Received OCA file");
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
            .body(serde_json::to_string(&vec![error]).unwrap());
    }
    let cached = app_state.cache.get(&ocafile);

    let result = match cached {
        Ok(Some(cached_said)) => {
            serde_json::json!({
                "success": true,
                "said": cached_said,
            })
        }
        Ok(None) => {
            let built_result = {
                app_state
                    .facade
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .build_from_ocafile(ocafile.clone(), app_state.overlayfile_registry.clone())
            };

            match built_result {
                Ok(oca_bundle) => {
                    let said = oca_bundle.digest.clone();
                    if let Err(e) = app_state.cache.insert(&ocafile, oca_bundle.digest.unwrap()) {
                        return HttpResponse::InternalServerError()
                            .content_type(ContentType::json())
                            .body(e.to_string());
                    };
                    serde_json::json!({
                        "success": true,
                        "said": said.unwrap(),
                    })
                }
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
                .body(e.to_string());
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

    // TODO compute and fill digest before serialization?? or does serialization handle this?
    let result = serde_json::json!({
        "r": search_result.records.iter().map(|r| {
            serde_json::json!({
                "oca_bundle":
                    serde_json::to_string(&r.oca_bundle).unwrap(),
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
    let said = match SelfAddressingIdentifier::from_str(&said_str) {
        Ok(said) => said,
        Err(e) => {
            return HttpResponse::UnprocessableEntity()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&vec![format!("Invalid SAID: {}", e)]).unwrap())
        }
    };

    let with_dependencies = query_params.w.unwrap_or(false);
    info!(
        "Received request for OCA bundle: {} include dependencies: {}",
        said_str, with_dependencies
    );

    // Lock once
    let facade = app_state.facade.lock().unwrap_or_else(|e| e.into_inner());

    let result = if with_dependencies {
        // Full bundle + dependencies
        match facade.get_oca_bundle_set(said) {
            Ok(bundle_set) => {
                let version = bundle_set.bundle.version.clone();
                serde_json::to_string(&serde_json::json!({
                        "v": version,
                        "bundle": bundle_set.bundle,
                        "dependencies":  bundle_set.dependencies
                }))
                .expect("Failed to serialize bundle set")
            }
            Err(errors) => serde_json::to_string(&serde_json::json!({
                "success": false,
                "errors": errors,
            }))
            .expect("Failed to serialize errors"),
        }
    } else {
        // Just the bundle (no dependencies)
        match facade.get_oca_bundle(said) {
            Ok(oca_bundle) => {
                let version = oca_bundle.version.clone();
                serde_json::to_string(&serde_json::json!({
                    "v": version,
                    "bundle": OCABundle::from(oca_bundle),
                    "dependencies": Vec::<serde_json::Value>::new(),
                }))
                .expect("Failed to serialize bundle")
            }
            Err(errors) => serde_json::to_string(&serde_json::json!({
                "success": false,
                "errors": errors,
            }))
            .expect("Failed to serialize errors"),
        }
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
        Err(e) => return HttpResponse::UnprocessableEntity()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&vec![format!("Invalid SAID: {}", e)]).unwrap()),
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

#[derive(Deserialize)]
pub struct DataEntryQuery {
    format: Option<String>,
    labels: Option<String>,
    metadata: Option<String>,
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

pub async fn get_oca_data_entry(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    query: web::Query<DataEntryQuery>,
) -> HttpResponse {
    let configuration = crate::configuration::get_configuration()
        .map_err(actix_web::error::ErrorInternalServerError)
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()));

    if let Err(resp) = configuration {
        return resp;
    }
    let configuration = configuration.unwrap();

    let data_entries_path = configuration
        .application
        .data_entries_path
        .unwrap_or_else(|| "/tmp".to_string());

    let said_str = req.match_info().get("said").unwrap().to_string();
    let said = match SelfAddressingIdentifier::from_str(&said_str) {
        Ok(said) => said,
        Err(e) => {
            return HttpResponse::UnprocessableEntity().body(format!("Invalid SAID: {}", e));
        }
    };

    let bundle_model = match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_oca_bundle(said.clone())
    {
        Ok(bundle) => bundle,
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.first().unwrap().clone());
        }
    };

    let bundle_set = match app_state
        .facade
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_oca_bundle_set(said.clone())
    {
        Ok(bundle_set) => bundle_set,
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.first().unwrap().clone());
        }
    };

    let mut deps_index = oca_data_entry::DependencyIndex::default();
    for dep in bundle_set.dependencies.iter() {
        if let Some(dep_said) = &dep.digest {
            deps_index.by_said.insert(dep_said.to_string(), dep.clone());
        }
    }

    let extract_options = oca_data_entry::ExtractOptions {
        label_lang: query.labels.clone(),
        metadata_lang: query.metadata.clone(),
    };
    let schema = match oca_data_entry::entry_schema_from_bundle_with_deps(
        &bundle_model,
        &deps_index,
        &app_state.overlayfile_registry,
        &extract_options,
    ) {
        Ok(schema) => schema,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let format = query.format.clone().unwrap_or_else(|| "xlsx".to_string());
    match format.as_str() {
        "csv" => {
            let mut out = Vec::new();
            let opts = oca_data_entry::CsvOptions {
                include_metadata_row: query.metadata.is_some(),
                label_lang: query.labels.clone(),
                metadata_lang: query.metadata.clone(),
            };
            if let Err(e) = oca_data_entry::write_csv(&schema, &mut out, &opts) {
                return HttpResponse::InternalServerError().body(e.to_string());
            }
            HttpResponse::Ok().content_type("text/csv").body(out)
        }
        "xlsx" => {
            let output_path = Path::new(&data_entries_path).join(format!("{}.xlsx", said_str));
            let opts = oca_data_entry::XlsxOptions {
                include_metadata_row: query.metadata.is_some(),
                label_lang: query.labels.clone(),
                metadata_lang: query.metadata.clone(),
            };
            if let Err(e) = oca_data_entry::write_xlsx(&schema, &output_path, &opts) {
                return HttpResponse::InternalServerError().body(e.to_string());
            }
            match actix_files::NamedFile::open(output_path) {
                Ok(file) => file.into_response(&req),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        _ => HttpResponse::BadRequest().body("format must be csv or xlsx"),
    }
}
