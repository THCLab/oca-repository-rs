use std::path::Path;

use base64::{prelude::BASE64_STANDARD, Engine};
use kv::{Bucket, Config, Json, Store};
use said::SelfAddressingIdentifier;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct OCAFilesCache {
    store: Store,
}

impl OCAFilesCache {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, kv::Error> {
        let cfg = Config::new(path);
        let store = Store::new(cfg)?;
        Ok(Self { store })
    }

    pub fn insert(&self, oca_file: &str, said: SelfAddressingIdentifier) -> Result<(), kv::Error> {
        let bucket: Bucket<String, Json<SelfAddressingIdentifier>> =
            self.store.bucket(Some("already_built"))?;
        let hash = compute_hash(oca_file);
        bucket.set(&hash, &Json(said))?;
        bucket.flush()?;
        Ok(())
    }

    pub fn get(&self, oca_file: &str) -> Result<Option<SelfAddressingIdentifier>, kv::Error> {
        let bucket: Bucket<String, Json<SelfAddressingIdentifier>> =
            self.store.bucket(Some("already_built"))?;
        let hash = compute_hash(oca_file);
        Ok(bucket.get(&hash)?.map(|el| el.0))
    }
}

pub fn compute_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    BASE64_STANDARD.encode(result)
}
