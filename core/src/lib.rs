use risc0_zkvm::sha::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {
    pub image: Vec<u8>,
    pub hash: Digest,
}

impl Input {
    pub fn new(image: &[u8]) -> Self {
        Self {
            image: image.to_vec(),
            hash: *risc0_zkvm::sha::Impl::hash_bytes(&image),
        }
    }

    pub fn read_from_file(fname: &str) -> Self {
        Self::new(&std::fs::read(fname).expect("Could not read file!"))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Output {
    pub image: Vec<u8>,
    pub hash: Digest,
}

impl Output {
    pub fn save_image_to_file(&self, fname: &str) {
        let mut file = File::create(fname).expect("Could not create file!");

        file.write_all(&self.image)
            .expect("Cannot write to the file!");
    }
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
}
