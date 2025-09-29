use risc0_zkvm::guest::env;
use shared::{
    layer::Layer,
    transformations::{Region, Transformation},
};

fn main() {
    let len: u32 = env::read();
    let mut image = vec![0u8; len as usize];
    env::read_slice(&mut image);

    let mut layer = Layer::new(&image).expect("Failed to create layer");

    layer
        .apply_transformation(Transformation::crop(Region {
            x: layer.image.width() * 3 / 8,
            y: layer.image.height() * 3 / 8,
            width: layer.image.width() / 4,
            height: layer.image.height() / 4,
        }))
        .expect("Failed to apply transformation");

    let image_out = layer.to_bytes().expect("Failed to convert image to bytes");

    env::commit_slice(image_out.as_slice());
}
