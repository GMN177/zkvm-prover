use risc0_zkvm::guest::{
    env,
    sha::{Impl, Sha256},
};
use shared::{layer::Layer, transformations::Transformation};

fn main() {
    let len: u32 = env::read();
    let mut image = vec![0u8; len as usize];
    env::read_slice(&mut image);

    let input_hash = *Impl::hash_bytes(&image);

    let mut layer = Layer::new(&image).expect("Failed to create layer");

    layer
        .apply_transformation(Transformation::resize2(
            layer.image.width() / 4,
            layer.image.height() / 4,
        ))
        .expect("Failed to apply transformation");

    let image_out = layer.to_bytes().expect("Failed to convert image to bytes");

    env::commit_slice(input_hash.as_bytes());
    env::commit_slice(image_out.as_slice());
}
