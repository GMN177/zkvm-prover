use methods::{METHOD_ELF, METHOD_ID};
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
