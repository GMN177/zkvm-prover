use risc0_zkvm::guest::{
    env,
    sha::{Impl, Sha256},
};
use shared::{layer::Layer, Input, Output};

fn main() {
    let input: Input = env::read();

    let input_hash = *Impl::hash_bytes(input.image.as_slice());

    let mut layer = Layer::new(input.image.as_slice()).expect("Failed to create layer");

    layer
        .apply_transformation(input.transformation)
        .expect("Failed to apply transformation");

    let image = layer.to_bytes().expect("Failed to convert image to bytes");

    let output_hash = *Impl::hash_bytes(image.as_slice());

    let output = Output {
        image,
        input_hash,
        output_hash,
    };

    env::commit(&output);
}
