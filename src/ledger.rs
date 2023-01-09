use microledger::{
    controlling_identifier::ControllingIdentifier,
    microledger::MicroLedger as MicroLedgerRaw,
    seal_bundle::{SealBundle, SealData},
    signature::Signature,
    Serialization,
};

use keri::{
    derivation::{basic::Basic, self_signing::SelfSigning},
    keys::{PrivateKey, PublicKey},
};

pub trait Ledger {
    fn add_block(&mut self, payload: &str, private_key: PrivateKey) -> Result<(), String>;
    fn init(ledger_str: Option<&str>, public_key: PublicKey) -> Self
    where
        Self: Sized;
    fn to_string(&self) -> Result<String, String>;
}

pub struct MicroLedger {
    ledger: Option<MicroLedgerRaw>,
    controlling_identifier: ControllingIdentifier,
}

impl Ledger for MicroLedger {
    fn init(ledger_str: Option<&str>, public_key: PublicKey) -> Self {
        let controlling_identifier =
            ControllingIdentifier::Basic(Basic::Ed25519.derive(public_key));

        match ledger_str {
            Some(ml_str) => Self {
                ledger: Some(serde_json::from_str::<MicroLedgerRaw>(ml_str).unwrap()),
                controlling_identifier,
            },
            None => Self {
                ledger: Some(MicroLedgerRaw::new()),
                controlling_identifier,
            },
        }
    }

    fn add_block(&mut self, payload: &str, private_key: PrivateKey) -> Result<(), String> {
        if let Some(ref ledger) = self.ledger {
            let seal_bundle = SealBundle::new().attach(SealData::AttachedData(payload.to_string()));

            let block =
                ledger.pre_anchor_block(vec![self.controlling_identifier.clone()], &seal_bundle);

            let signature_raw = private_key.sign_ed(&block.serialize()).unwrap();
            let s = Signature::SelfSigning(SelfSigning::Ed25519Sha512.derive(signature_raw));
            let signed_block = block.to_signed_block(vec![s], &seal_bundle);

            self.ledger = Some(ledger.anchor(signed_block).unwrap());
        }

        Ok(())
    }

    fn to_string(&self) -> Result<String, String> {
        if let Some(ref ledger) = self.ledger {
            Ok(serde_json::to_string(ledger).unwrap())
        } else {
            Err("Miising ledger".to_string())
        }
    }
}
