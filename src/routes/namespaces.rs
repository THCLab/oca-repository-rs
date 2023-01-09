use crate::data_storage::DataStorage;
use crate::ledger::Ledger;
use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use oca_rust::state::oca::{overlay, OCA};
use serde::{Deserialize, Serialize};

use keri::keys::{PrivateKey, PublicKey};
use rand::rngs::OsRng;

use meilisearch_sdk::client::*;

fn generate_key_pair() -> ed25519_dalek::Keypair {
    ed25519_dalek::Keypair::generate(&mut OsRng {})
}

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
    fn new(capture_base_said: String, name: String, description: String) -> Self {
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
    let namespace_value = item.get("namespace").unwrap();

    if let serde_json::Value::String(namespace) = namespace_value {
        let keypair = generate_key_pair();

        db.insert(
            &format!("namespace.{namespace}.public_key"),
            keypair.public.as_bytes(),
        )
        .unwrap();
        db.insert(
            &format!("namespace.{namespace}.secret_key"),
            keypair.secret.as_bytes(),
        )
        .unwrap();
    }

    HttpResponse::Ok().finish()
}

pub async fn add_bundle(
    db: web::Data<Box<dyn DataStorage>>,
    search_engine_client: web::Data<Box<Client>>,
    item: web::Json<OCA>,
    req: HttpRequest,
) -> HttpResponse {
    let namespace = req.match_info().get("namespace").unwrap();

    let pk = db
        .get(&format!("namespace.{namespace}.public_key"))
        .unwrap()
        .unwrap();
    let sk = db
        .get(&format!("namespace.{namespace}.secret_key"))
        .unwrap()
        .unwrap();

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

    ledger.add_block(payload.as_str(), private_key).unwrap();

    db.insert(
        &format!("namespace.{namespace}.{}", item.capture_base.said),
        ledger.to_string().unwrap().as_bytes(),
    )
    .unwrap();

    println!("{:?}", ledger.to_string().unwrap());

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
