use host::full_prove;
use shared::{transformations::Transformation, Input};
use std::time::Instant;

// const INPUT_FILE: &str = "res/sample_100x100_rgb.png";
// const OUTPUT_FILE: &str = "res/sample_100x100_rotate90.png";
// const PROOF_G16_FILE: &str = "res/proof_g16.json";

fn main() {
    test_prove();
}

fn test_prove() {
    let tests = [
        (
            "res/sample_200x200_rgb.png",
            "res/sample_200x200_resize_10x10.png",
            Transformation::resize(10, 10),
        ),
        (
            "res/sample_200x200_rgb.png",
            "res/sample_200x200_resize_50x50.png",
            Transformation::resize(50, 50),
        ),
        (
            "res/sample_200x200_rgb.png",
            "res/sample_200x200_resize_100x100.png",
            Transformation::resize(100, 100),
        ),
        (
            "res/sample_200x200_rgb.png",
            "res/sample_200x200_resize_200x200.png",
            Transformation::resize(200, 200),
        ),
        (
            "res/sample_200x200_rgb.png",
            "res/sample_200x200_resize_400x400.png",
            Transformation::resize(400, 400),
        ),
    ];

    for (input_file, output_file, transformation) in tests {
        println!("Proving for {} -> {}", input_file, output_file);
        let before = Instant::now();
        let input = Input::new(&std::fs::read(input_file).unwrap(), transformation);
        let proof = full_prove(input).expect("Proving failed");
        println!("Proving time: {:.2?}", before.elapsed());
        proof.save_as_json(format!("{}.json", output_file).as_str());
        let output = proof.get_journal();
        output.save_image_to_file(output_file);
    }
}

// fn verify() {
//     let proof = Proof::read_from_json(PROOF_G16_FILE);

//     proof.verify();

//     let pruned = prune_receipt(&proof.receipt);

//     println!("{:#?}", pruned);

//     pruned.verify(proof.id).expect("Verification failed");

//     let output = proof.get_journal();

//     output.save_image_to_file(OUTPUT_FILE);
// }
