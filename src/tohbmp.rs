use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom, Cursor};
use image::ImageFormat;
use indicatif::{ProgressBar, ProgressStyle};

pub fn convert_to_hbmp(input_file: &str, output_file: &str, header_file: &str) -> io::Result<()> {
    // Read the input image
    let img = image::open(input_file).expect("Failed to open input image");

    // Convert the image to BMP format using a Cursor
    let mut bmp_data = Cursor::new(Vec::new());
    img.write_to(&mut bmp_data, ImageFormat::Bmp).expect("Failed to write BMP data");

    // Create a progress bar
    let pb = ProgressBar::new(bmp_data.get_ref().len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?  // Convert TemplateError to io::Error
            .progress_chars("#>-")
    );

    // Write BMP data to file
    let mut bmp_file = File::create(output_file)?;
    bmp_file.write_all(bmp_data.get_ref())?;
    pb.inc(bmp_data.get_ref().len() as u64);
    pb.finish_with_message("Conversion to BMP complete");

    // Remove BMP header and store it separately
    let header_size = 54; // BMP header size is 54 bytes
    let mut bmp_file = File::open(output_file)?;
    let mut header = vec![0; header_size];
    let mut bmp_body = Vec::new();

    bmp_file.read_exact(&mut header)?;
    bmp_file.seek(SeekFrom::Start(header_size as u64))?;
    bmp_file.read_to_end(&mut bmp_body)?;

    // Write header to a separate file
    let mut header_file = File::create(header_file)?;
    header_file.write_all(&header)?;

    // Write BMP body back to the BMP file (without the header)
    let mut bmp_file = File::create(output_file)?;
    bmp_file.write_all(&bmp_body)?;

    Ok(())
}
