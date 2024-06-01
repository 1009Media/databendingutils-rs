use image::{ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{BufReader, Read};

pub fn convert_from_sbr(input_file: &str, header_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the header file for width and height
    let mut hf = BufReader::new(File::open(header_file)?);
    let mut header = String::new();
    hf.read_to_string(&mut header)?;
    let (width, height): (u32, u32) = {
        let dims: Vec<&str> = header.trim().split(',').collect();
        (dims[0].parse()?, dims[1].parse()?)
    };

    // Calculate the size of each color channel block
    let block_size = (width * height) as usize;

    // Read the SBR file
    let mut f = BufReader::new(File::open(input_file)?);
    let mut r_bytes = vec![0; block_size];
    let mut g_bytes = vec![0; block_size];
    let mut b_bytes = vec![0; block_size];
    f.read_exact(&mut r_bytes)?;
    f.read_exact(&mut g_bytes)?;
    f.read_exact(&mut b_bytes)?;

    // Create ImageBuffer from bytes
    let mut img_buffer = ImageBuffer::new(width, height);

    let bar = ProgressBar::new((width * height) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?  // Convert TemplateError to io::Error
            .progress_chars("#>-")
    );

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let idx = (y * width + x) as usize;
        *pixel = Rgb([r_bytes[idx], g_bytes[idx], b_bytes[idx]]);
        bar.inc(1);
    }

    bar.finish();

    // Save the image in the desired format
    img_buffer.save(output_file)?;
    
    Ok(())
}
