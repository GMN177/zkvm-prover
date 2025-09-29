use risc0_zkvm::guest::{
    env,
    sha::{Impl, Sha256},
};

fn main() {
    let len: u32 = env::read();
    let mut image = vec![0u8; len as usize];
    env::read_slice(&mut image);

    let input_hash = *Impl::hash_bytes(&image);

    env::commit_slice(input_hash.as_bytes());
}
