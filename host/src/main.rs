use core::{Input, Proof};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use std::time::Instant;

#[allow(unused_imports)]
use methods::{GRAYSCALE_ELF, GRAYSCALE_ID, ROTATE90_ELF, ROTATE90_ID};

const INPUT_FILE: &str = "res/sample_100x100_rgb.png";
const OUTPUT_FILE: &str = "res/sample_100x100_rotate90.png";
const PROOF_FILE: &str = "res/proof.json";
const PROOF_G16_FILE: &str = "res/proof_g16.json";

fn main() {
    println!("Proving...");
    let before = Instant::now();
    prove();
    println!("Proving time: {:.2?}", before.elapsed());

    println!("Compressing...");
    let before = Instant::now();
    compress();
    println!("Compressing time: {:.2?}", before.elapsed());

    println!("Verifying...");
    let before = Instant::now();
    verify();
    println!("Verifying time: {:.2?}", before.elapsed());
}

fn prove() {
    let input = Input::read_from_file(INPUT_FILE);

    let env = ExecutorEnv::builder()
        .write(&input)
        .expect("Failed to write input")
        .build()
        .expect("Failed to build executor");

    let prover = default_prover();

    let prove_info = prover.prove(env, ROTATE90_ELF).expect("Proving failed");

    let receipt = prove_info.receipt;

    let proof = Proof::new(receipt, ROTATE90_ID);

    proof.save_as_json(PROOF_FILE);
}

fn compress() {
    let proof = Proof::read_from_json(PROOF_FILE);

    let prover = default_prover();

    let prover_opts = ProverOpts::groth16();

    let compressed_receipt = prover
        .compress(&prover_opts, &proof.receipt)
        .expect("Compression failed");

    assert_eq!(
        compressed_receipt.journal, proof.receipt.journal,
        "Journal mismatch after decompression"
    );

    let proof = Proof::new(compressed_receipt, proof.id);

    proof.save_as_json(PROOF_G16_FILE);
}

fn verify() {
    let proof = Proof::read_from_json(PROOF_G16_FILE);

    proof.verify();

    let output = proof.get_journal();

    output.save_image_to_file(OUTPUT_FILE);
}
