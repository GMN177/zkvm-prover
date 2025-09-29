use methods::{CROP_ELF, CROP_ID};
use methods::{GRAYSCALE_ELF, GRAYSCALE_ID};
use methods::{HASH_ELF, HASH_ID};
use methods::{IDENTITY2_ELF, IDENTITY2_ID};
use methods::{IDENTITY_ELF, IDENTITY_ID};
use methods::{METHOD_ELF, METHOD_ID};
use methods::{RESIZE_ELF, RESIZE_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use shared::{Input, Proof};

pub fn full_prove(input: Input) -> Result<Proof, String> {
    let env = ExecutorEnv::builder()
        .write(&input)
        .map_err(|e| format!("Failed to write input: {}", e))?
        .build()
        .map_err(|e| format!("Failed to build executor env: {}", e))?;

    let prover = default_prover();

    let prover_opts = ProverOpts::groth16();

    let prove_info = prover
        .prove_with_opts(env, METHOD_ELF, &prover_opts)
        .map_err(|e| format!("Prover failed: {}", e))?;

    Ok(Proof::new(prove_info.receipt, METHOD_ID))
}

pub fn full_prove_generic(elf: &[u8], id: [u32; 8], input: Vec<u8>) -> Result<Proof, String> {
    let env = ExecutorEnv::builder()
        .write_frame(&input.as_slice())
        .build()
        .map_err(|e| format!("Failed to build executor env: {}", e))?;

    let prover = default_prover();

    let prover_opts = ProverOpts::groth16();

    let prove_info = prover
        .prove_with_opts(env, elf, &prover_opts)
        .map_err(|e| format!("Prover failed: {}", e))?;

    Ok(Proof::new(prove_info.receipt, id))
}

pub fn full_prove_hash(input: Vec<u8>) -> Result<Proof, String> {
    full_prove_generic(HASH_ELF, HASH_ID, input)
}

pub fn full_prove_resize(input: Vec<u8>) -> Result<Proof, String> {
    full_prove_generic(RESIZE_ELF, RESIZE_ID, input)
}

pub fn full_prove_grayscale(input: Vec<u8>) -> Result<Proof, String> {
    full_prove_generic(GRAYSCALE_ELF, GRAYSCALE_ID, input)
}

pub fn full_prove_crop(input: Vec<u8>) -> Result<Proof, String> {
    full_prove_generic(CROP_ELF, CROP_ID, input)
}

pub fn full_prove_identity(input: Vec<u8>) -> Result<Proof, String> {
    full_prove_generic(IDENTITY_ELF, IDENTITY_ID, input)
}

pub fn full_prove_identity2(input: Vec<u8>) -> Result<Proof, String> {
    full_prove_generic(IDENTITY2_ELF, IDENTITY2_ID, input)
}
