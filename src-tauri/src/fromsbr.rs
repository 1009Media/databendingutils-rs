use image::{ImageBuffer, Rgb};
use std::fs::File;
use std::io::{BufReader, Read};

pub fn convert_from_sbr(input_file: &str, header_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut hf = BufReader::new(File::open(header_file)?);
    let mut header = String::new();
    hf.read_to_string(&mut header)?;
    let (width, height): (u32, u32) = {
        let dims: Vec<&str> = header.trim().split(',').collect();
        (dims[0].parse()?, dims[1].parse()?)
    };

    let block_size = (width * height) as usize;
    let mut f = BufReader::new(File::open(input_file)?);
    let mut r_bytes = vec![0; block_size];
    let mut g_bytes = vec![0; block_size];
    let mut b_bytes = vec![0; block_size];
    f.read_exact(&mut r_bytes)?;
    f.read_exact(&mut g_bytes)?;
    f.read_exact(&mut b_bytes)?;

    let mut img_buffer = ImageBuffer::new(width, height);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let idx = (y * width + x) as usize;
        *pixel = Rgb([r_bytes[idx], g_bytes[idx], b_bytes[idx]]);
    }

    img_buffer.save(output_file)?;

    Ok(())
}
