use risc0_zkvm::guest::env;

fn main() {
    let image: Vec<u8> = env::read();
    env::commit(&image);
}
