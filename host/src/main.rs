use host::{full_prove, full_prove_crop, full_prove_grayscale, full_prove_hash, full_prove_resize};
use shared::{transformations::Transformation, Input, Proof};
use std::{fs::File, io::Write, time::Instant};

fn main() {
    //verify();
    test_prove_by_size();
    //test_prove();
}

fn test_prove_by_size() {
    let tests = [
        //"res/sample_10x10_rgb.tif",
        "res/sample_100x100_rgb.png",
        "res/sample_200x200_rgb.png",
        "res/sample_400x400_rgb.png",
        "res/sample_800x800_rgb.png",
        "res/sample_1600x1600_rgb.png",
    ];

    let mut file =
        File::create("res/proving_times_all_profiler.csv").expect("Could not create file");

    writeln!(file, "Input File,Hash Proving Time (ms),Crop Proving Time (ms),Grayscale Proving Time (ms),Resize Proving Time (ms)")
        .expect("Could not write header");

    for input_file in tests {
        println!("Proving for {}", input_file);
        let image = std::fs::read(input_file).unwrap();

        let input = image.clone();
        let before = Instant::now();
        let proof = full_prove_hash(input).expect("Proving failed");
        let hash_duration = before.elapsed();
        println!("Hash proving time: {:.2?}", hash_duration);
        proof.save_as_json(format!("{}.hash.json", input_file).as_str());

        let input = image.clone();
        let before = Instant::now();
        let proof = full_prove_crop(input).expect("Proving failed");
        let crop_duration = before.elapsed();
        println!("Crop proving time: {:.2?}", crop_duration);
        proof.save_as_json(format!("{}.crop.json", input_file).as_str());

        let input = image.clone();
        let before = Instant::now();
        let proof = full_prove_grayscale(input).expect("Proving failed");
        let grayscale_duration = before.elapsed();
        println!("Grayscale proving time: {:.2?}", grayscale_duration);
        proof.save_as_json(format!("{}.grayscale.json", input_file).as_str());

        let input = image.clone();
        let before = Instant::now();
        let proof = full_prove_resize(input).expect("Proving failed");
        let resize_duration = before.elapsed();
        println!("Resize proving time: {:.2?}", resize_duration);
        proof.save_as_json(format!("{}.resize.json", input_file).as_str());

        writeln!(
            file,
            "{},{},{},{},{}",
            input_file,
            hash_duration.as_millis(),
            crop_duration.as_millis(),
            grayscale_duration.as_millis(),
            resize_duration.as_millis()
        )
        .expect("Could not write data");
    }
}

// fn test_prove() {
//     let tests = [
//         (
//             "res/sample_200x200_rgb.png",
//             "res/sample_200x200_resize_10x10.png",
//             Transformation::resize(10, 10),
//         ),
//         (
//             "res/sample_200x200_rgb.png",
//             "res/sample_200x200_resize_50x50.png",
//             Transformation::resize(50, 50),
//         ),
//         (
//             "res/sample_200x200_rgb.png",
//             "res/sample_200x200_resize_100x100.png",
//             Transformation::resize(100, 100),
//         ),
//         (
//             "res/sample_200x200_rgb.png",
//             "res/sample_200x200_resize_200x200.png",
//             Transformation::resize(200, 200),
//         ),
//         (
//             "res/sample_200x200_rgb.png",
//             "res/sample_200x200_resize_400x400.png",
//             Transformation::resize(400, 400),
//         ),
//     ];

//     for (input_file, output_file, transformation) in tests {
//         println!("Proving for {} -> {}", input_file, output_file);
//         let before = Instant::now();
//         let input = Input::new(&std::fs::read(input_file).unwrap(), transformation);
//         let proof = full_prove(input).expect("Proving failed");
//         println!("Proving time: {:.2?}", before.elapsed());
//         proof.save_as_json(format!("{}.json", output_file).as_str());
//         let output = proof.get_journal();
//         output.save_image_to_file(output_file);
//     }
// }

// fn verify() {
//     let proof = Proof::read_from_json("res/sample_100x100_rgb.png.hash.json");

//     let before = Instant::now();
//     proof.verify();
//     let after = before.elapsed();
//     println!("Verification time: {:.2?}", after);

//     //let pruned = prune_receipt(&proof.receipt);

//     //println!("{:#?}", pruned);

//     //pruned.verify(proof.id).expect("Verification failed");

//     //let output = proof.get_journal();

//     //output.save_image_to_file("res/sample_100x100_rgb.hash.png");
// }
