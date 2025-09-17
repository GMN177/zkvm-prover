use risc0_zkvm::sha::Digest;
use risc0_zkvm::sha::Digestible;
use risc0_zkvm::{serde::to_vec, Receipt};
use risc0_zkvm::{Groth16Receipt, MaybePruned};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

pub mod layer;
pub mod transformations;

use transformations::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Input {
    pub image: Vec<u8>,
    pub transformation: Transformation,
}

impl Input {
    pub fn new(image: &[u8], transformation: Transformation) -> Self {
        Self {
            image: image.to_vec(),
            transformation,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
    pub image: Vec<u8>,
    pub input_hash: Digest,
    pub output_hash: Digest,
}

impl Output {
    pub fn save_image_to_file(&self, fname: &str) {
        let mut file = File::create(fname).expect("Could not create file!");

        file.write_all(&self.image)
            .expect("Cannot write to the file!");
    }
}

pub fn prune_receipt(receipt: &risc0_zkvm::Receipt) -> risc0_zkvm::Receipt {
    let unkown_receipt = receipt.inner.groth16().unwrap().clone().into_unknown();

    Receipt::new(
        Groth16Receipt::new(
            unkown_receipt.seal.clone(),
            MaybePruned::Pruned(unkown_receipt.claim.digest()),
            unkown_receipt.verifier_parameters, //Groth16ReceiptVerifierParameters::default().digest(),
        )
        .into(),
        receipt.journal.clone().bytes,
    )
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Proof {
    pub receipt: risc0_zkvm::Receipt,
    pub id: [u32; 8],
}

impl Proof {
    pub fn new(receipt: risc0_zkvm::Receipt, id: [u32; 8]) -> Self {
        Self { receipt, id }
    }

    pub fn read_from_json(fname: &str) -> Self {
        let file = File::open(fname).expect("Could not open file!");
        serde_json::from_reader(file).expect("Could not parse JSON!")
    }

    pub fn save_as_json(&self, fname: &str) {
        let proof_as_json = serde_json::to_string(self).expect("Could not serialize to JSON!");

        let mut file = File::create(fname).expect("Could not create file!");

        file.write_all(proof_as_json.as_bytes())
            .expect("Cannot write to the file!");
    }

    pub fn verify(&self) {
        self.receipt.verify(self.id).expect("Verification failed");
    }

    pub fn get_journal(&self) -> Output {
        self.receipt
            .journal
            .decode()
            .expect("Could not decode journal")
    }

    pub fn test(&self) {
        let output = self.get_journal();
        //Serializer::serialize(output).expect("Could not serialize hash!");
        to_vec(&output).expect("Could not serialize output!");
    }
}
