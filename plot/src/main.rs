use csv::ReaderBuilder;
use plotters::prelude::*;
use std::error::Error;

fn read_file_size(indentifier: &str) -> Result<u64, Box<dyn Error>> {
    let metadata = std::fs::metadata("res/sample_".to_string() + indentifier + "_rgb.png")?;
    Ok(metadata.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let mut rdr = ReaderBuilder::new().from_path("res/proving_times_all.csv")?;

    let mut labels = Vec::new();
    let mut file_size = Vec::new();
    let mut hash_times = Vec::new();
    let mut resize_times = Vec::new();
    let mut grayscale_times = Vec::new();
    let mut crop_times = Vec::new();

    for result in rdr.records() {
        let record = result?;
        labels.push(record[0].to_string());
        file_size.push(read_file_size(&record[0])?);
        hash_times.push(record[1].parse::<u64>()? / 1_000);
        resize_times.push(record[2].parse::<u64>()? / 1_000);
        grayscale_times.push(record[3].parse::<u64>()? / 1_000);
        crop_times.push(record[4].parse::<u64>()? / 1_000);
    }

    // Create drawing area
    let root = BitMapBackend::new("res/proving_times3.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = *[
        hash_times.iter().max().unwrap(),
        resize_times.iter().max().unwrap(),
        grayscale_times.iter().max().unwrap(),
        crop_times.iter().max().unwrap(),
    ]
    .iter()
    .max()
    .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Proving Times", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(
            0..(file_size.len() - 1),
            (10..(max_y + max_y / 10)).log_scale().base(2.0),
        )?;

    chart
        .configure_mesh()
        .x_labels(file_size.len())
        .x_label_formatter(&|idx| file_size[*idx].to_string())
        //.x_label_formatter(&|idx| file_size[*idx].clone())
        //.x_label_angle(45)
        .y_desc("Time (s)")
        .x_desc("Input Image (bytes)")
        .draw()?;

    // Plot series
    chart
        .draw_series(LineSeries::new((0..).zip(hash_times.iter().cloned()), &RED))?
        .label("Hash")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            (0..).zip(resize_times[0..resize_times.len() - 1].iter().cloned()),
            &BLUE,
        ))?
        .label("Resize")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(
            (0..).zip(
                grayscale_times[0..grayscale_times.len() - 1]
                    .iter()
                    .cloned(),
            ),
            &GREEN,
        ))?
        .label("Grayscale")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(LineSeries::new(
            (0..).zip(crop_times[0..crop_times.len() - 1].iter().cloned()),
            &BLACK,
        ))?
        .label("Crop")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    Ok(())
}
