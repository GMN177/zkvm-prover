// use methods::{CROP_ELF, CROP_ID};
// use methods::{GRAYSCALE_ELF, GRAYSCALE_ID};
// use methods::{HASH_ELF, HASH_ID};
// use methods::{IDENTITY2_ELF, IDENTITY2_ID};
// use methods::{IDENTITY_ELF, IDENTITY_ID};
// use methods::{RESIZE_ELF, RESIZE_ID};
use methods::HASHRESIZE_ELF;
use risc0_zkvm::{default_executor, ExecutorEnv};

// const images: &[&str] = &[
//     "res/sample_100x100_rgb.png",
//     "res/sample_200x200_rgb.png",
//     "res/sample_400x400_rgb.png",
//     "res/sample_800x800_rgb.png",
//     "res/sample_1600x1600_rgb.png",
// ];

// const elfs: &[(&[u8], &[u32; 8], &str)] = &[
//     (HASH_ELF, &HASH_ID, "hash"),
//     (RESIZE_ELF, &RESIZE_ID, "resize"),
//     (GRAYSCALE_ELF, &GRAYSCALE_ID, "grayscale"),
//     (CROP_ELF, &CROP_ID, "crop"),
//     (IDENTITY_ELF, &IDENTITY_ID, "identity"),
//     (IDENTITY2_ELF, &IDENTITY2_ID, "identity2"),
// ];

fn main() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let input = std::fs::read("res/sample_100x100_rgb.png")
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    let env = ExecutorEnv::builder()
        .write_frame(&input.as_slice())
        //.map_err(|e| format!("Failed to write input: {}", e))?
        .build()
        .map_err(|e| format!("Failed to build executor env: {}", e))?;

    let exec = default_executor();

    let session_info = exec
        .execute(env, HASHRESIZE_ELF)
        .map_err(|e| format!("Executor failed: {}", e))?;

    println!("{}", session_info.cycles());

    Ok(())
}
