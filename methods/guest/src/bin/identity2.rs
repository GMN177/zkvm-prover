use risc0_zkvm::guest::env;
use shared::layer::Layer;

fn main() {
    let image: Vec<u8> = env::read();

    let layer = Layer::new(image.as_slice()).expect("Failed to create layer");

    let image_out = layer.to_bytes().expect("Failed to convert image to bytes");

    env::commit(&image_out);
}
