use std::collections::HashMap;
use rand::RngCore;
use oca_rs::data_storage::DataStorage;
use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse, HttpMessage};
use oca_rust::state::oca::OCA;
use serde::{Deserialize, Serialize};

use rand::rngs::OsRng;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

use meilisearch_sdk::client::*;


#[derive(Deserialize)]
pub struct SearchBundleQuery {
    q: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OCASearchIndex {
    uuid: String,
    capture_base_said: String,
    name: String,
    description: String,
}

impl OCASearchIndex {
    fn _new(capture_base_said: String, name: String, description: String) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4().to_string(),
            capture_base_said,
            name,
            description,
        }
    }
}

pub async fn add_namespace(
    db: web::Data<Box<dyn DataStorage>>,
    item: web::Json<serde_json::Value>,
) -> HttpResponse {
    let mut errors: Vec<String> = vec![];
    let mut result: HashMap<&str, serde_json::Value> = HashMap::new();

    if item.get("namespace").is_none() {
        errors.push("Namespace is required".to_string());
    }

    if let Some(serde_json::Value::String(namespace)) = item.get("namespace") {
        if db.get(&format!("namespace.{namespace}.public_key")).unwrap().is_some() {
            errors.push(format!("Namespace {namespace} already exists"));
        }

        if errors.is_empty() {
            let mut seed = [0u8; 32];
            OsRng.fill_bytes(&mut seed);

            let secret_key = ed25519_dalek::SecretKey::from_bytes(&seed).unwrap();
            let public_key = ed25519_dalek::PublicKey::from(&secret_key);

            db.insert(
                &format!("namespace.{namespace}.public_key"),
                public_key.as_bytes(),
            )
            .unwrap();

            const CUSTOM_ENGINE: engine::GeneralPurpose =
                engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
            let seed_b64 = CUSTOM_ENGINE.encode(seed);

            result.insert("token", serde_json::Value::String(seed_b64));
        }
    }

    if errors.is_empty() {
        result.insert("success", serde_json::Value::Bool(true));
    } else {
        result.insert("success", serde_json::Value::Bool(false));
        result.insert("errors", serde_json::Value::Array(errors.iter().map(|e| serde_json::Value::String(e.to_string())).collect()));
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

pub async fn get_namespace(
    db: web::Data<Box<dyn DataStorage>>,
    req: HttpRequest,
) -> HttpResponse {
    let result: HashMap<&str, serde_json::Value> = HashMap::new();
    let namespace = req.match_info().get("namespace").unwrap();
    let token = req.extensions().get::<String>().unwrap().clone();
    println!("req: {req:?}");
    println!("token: {}", token);

    let _public_key = db
        .get(&format!("namespace.{namespace}.public_key"))
        .unwrap()
        .unwrap();

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let _seed = CUSTOM_ENGINE.decode(token).unwrap();

    /*
    let mut ed25519_seed = [0u8; 32];
    OsRng.fill_bytes(&mut ed25519_seed);

    println!("b64: {b64_url}");
    println!("ed25519_seed: {:?}", String::from_utf8_lossy(&ed25519_seed));

    let seed_str = "AOs8-zNPPh0EhavdrCfCiTk9nGeO8e6VxUCzwdKXJAd0";
    println!("{:?}", CUSTOM_ENGINE.decode(seed_str).unwrap());

    let sec_key = ed25519_dalek::SecretKey::from_bytes(&ed25519_seed).unwrap();
    let pub_key = ed25519_dalek::PublicKey::from(&sec_key);
    println!("sk: {sec_key:?} = pub_key: {pub_key:?}");

    let seed: SeedPrefix = seed_str.parse().unwrap();
    let (pub_key, priv_key) = seed.derive_key_pair().unwrap();
    println!("pk: {pk:?} = pub_key: {pub_key:?}");
    println!("sk: {:?} = sec_key: {priv_key:?}", String::from_utf8_lossy(&sk));

    println!(
        "get_namespace: {}, params: {}",
        namespace,
        req.query_string(),
    );
    */
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

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

pub async fn get_oca_file_history(
    db: web::Data<Box<dyn DataStorage>>,
    req: HttpRequest,
) -> HttpResponse {
    let said = req.match_info().get("said").unwrap().to_string();

    let oca_facade = oca_rs::Facade::new(db.get_ref().clone());
    let result = match oca_facade.get_oca_bundle_steps(said) {
        Ok(oca_build_steps) => {
            serde_json::to_value(
                &oca_build_steps.iter().map(|s| {
                    serde_json::json!({
                        "from": s.parent_said,
                        "operation": s.command,
                    })
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

pub async fn add_bundle(
    db: web::Data<Box<dyn DataStorage>>,
    _search_engine_client: web::Data<Box<Client>>,
    _item: web::Json<OCA>,
    req: HttpRequest,
) -> HttpResponse {
    let namespace = req.match_info().get("namespace").unwrap();
    let token = req.extensions().get::<String>().unwrap().clone();

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    let seed = CUSTOM_ENGINE.decode(token).unwrap();
    let secret_key = ed25519_dalek::SecretKey::from_bytes(&seed).unwrap();
    let _public_key = ed25519_dalek::PublicKey::from(&secret_key);

    let _stored_public_key = db
        .get(&format!("namespace.{namespace}.public_key"))
        .unwrap()
        .unwrap();

    /*
    if public_key.as_bytes().to_vec() == stored_public_key {
        let bundle_digests = versioning::bundle::to_digests(&item);

        println!("bundle_digests: {bundle_digests:?}");
        println!("bundle_digests: {:?}", String::from_utf8_lossy(&bundle_digests));
        db.insert(
            &format!("namespace.{namespace}.bundle"),
            bundle_digests.as_slice(),
        )
        .unwrap();
    }
    */

    /*
    let public_key = PublicKey::new(pk);
    let private_key = PrivateKey::new(sk);

    let payload = String::from("OCA Bundle SAID"); // item.digest.to_string();

    let mut ledger = match db
        .get(&format!("namespace.{namespace}.{}", item.capture_base.said))
        .unwrap()
    {
        Some(value) => {
            let ledger_str = String::from_utf8(value).unwrap();
            crate::ledger::MicroLedger::init(Some(&ledger_str), public_key)
        }
        None => crate::ledger::MicroLedger::init(None, public_key),
    };

    let meta_overlays = item
        .overlays
        .iter()
        .filter(|o| o.overlay_type().contains("/meta/"))
        .map(|o| o.as_any().downcast_ref::<overlay::Meta>().unwrap())
        .collect::<Vec<&overlay::Meta>>();

    for meta_overlay in meta_overlays {
        search_engine_client
            .index("oca")
            .add_documents(
                &[OCASearchIndex::new(
                    item.capture_base.said.clone(),
                    meta_overlay.name.clone(),
                    meta_overlay.description.clone(),
                )],
                Some("uuid"),
            )
            .await
            .unwrap();
    }

    ledger.add_block(payload.as_str(), secret_key).unwrap();

    db.insert(
        &format!("namespace.{namespace}.{}", item.capture_base.said),
        ledger.to_string().unwrap().as_bytes(),
    )
    .unwrap();

    println!("{:?}", ledger.to_string().unwrap());
    */

    HttpResponse::Ok().finish()
}

pub async fn search_bundle(
    search_engine_client: web::Data<Box<Client>>,
    query: web::Query<SearchBundleQuery>,
) -> HttpResponse {
    let search_result = search_engine_client
        .index("oca")
        .search()
        .with_query(&query.q)
        .execute::<OCASearchIndex>()
        .await
        .unwrap();

    let result = search_result
        .hits
        .iter()
        .map(|h| &h.result)
        .collect::<Vec<&OCASearchIndex>>();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}
