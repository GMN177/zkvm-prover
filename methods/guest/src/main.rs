use core::{Input, Output};
use image::ImageReader;
use risc0_zkvm::guest::{
    env,
    sha::{Impl, Sha256},
};
use std::io::Cursor;

fn main() {
    let input: Input = env::read();

    let sha = *Impl::hash_bytes(input.image.as_slice());

    assert_eq!(sha, input.hash, "Hash mismatch");

    let img_reader = ImageReader::new(Cursor::new(input.image))
        .with_guessed_format()
        .expect("Failed to guess image format");

    let img_format = img_reader.format().expect("Failed to get image format");

    let img = img_reader.decode().expect("Failed to decode image");

    let grey_image = img.grayscale();

    let mut grey_image_output: Vec<u8> = Vec::new();

    grey_image
        .write_to(&mut Cursor::new(&mut grey_image_output), img_format)
        .expect("Failed to write grayscale image");

    let output = Output {
        image: grey_image_output,
        hash: input.hash,
    };

    env::commit(&output);
}
