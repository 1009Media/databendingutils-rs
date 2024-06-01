use image::{GenericImageView, Rgb};
use indicatif::ProgressBar;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn convert_to_sbr(input_file: &str, output_file: &str, header_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the image file
    let img = image::open(input_file)?;
    
    // Convert image to RGB
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();

    // Extract RGB channels
    let mut r = Vec::with_capacity((width * height) as usize);
    let mut g = Vec::with_capacity((width * height) as usize);
    let mut b = Vec::with_capacity((width * height) as usize);
    
    let bar = ProgressBar::new((width * height) as u64);

    for pixel in img.pixels() {
        r.push(pixel[0]);
        g.push(pixel[1]);
        b.push(pixel[2]);
        bar.inc(1);
    }

    bar.finish();

    // Write the SBR file
    let mut f = BufWriter::new(File::create(output_file)?);
    f.write_all(&r)?;
    f.write_all(&g)?;
    f.write_all(&b)?;

    // Write the header file with width and height
    let mut hf = BufWriter::new(File::create(header_file)?);
    writeln!(hf, "{},{}", width, height)?;

    Ok(())
}
