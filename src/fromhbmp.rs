use std::fs::File;
use std::io::{self, Read, BufReader, BufWriter};
use image::{ImageFormat, load_from_memory};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

pub fn convert_from_hbmp(
    input_file: &str,
    header_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the BMP header
    let mut header_file = BufReader::new(File::open(header_file)?);
    let mut header = Vec::new();
    header_file.read_to_end(&mut header)?;

    // Read the headless BMP data
    let mut input_file = BufReader::new(File::open(input_file)?);
    let mut bmp_body = Vec::new();
    input_file.read_to_end(&mut bmp_body)?;

    // Combine the header and body to form the complete BMP data
    let mut complete_bmp_data = Vec::new();
    complete_bmp_data.extend_from_slice(&header);
    complete_bmp_data.extend_from_slice(&bmp_body);
    
    // Load the BMP image from the complete data
    let img = load_from_memory(&complete_bmp_data).expect("Failed to load BMP image from memory");

    let bar = ProgressBar::new(bmp_body.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?  // Convert TemplateError to io::Error
            .progress_chars("#>-")
    );

    // Determine the output format based on the file extension
    let output_path = Path::new(output_file);
    let output_format = match output_path.extension().and_then(|ext| ext.to_str()) {
        Some("png") => ImageFormat::Png,
        Some("jpeg") | Some("jpg") => ImageFormat::Jpeg,
        Some("gif") => ImageFormat::Gif,
        Some("bmp") => ImageFormat::Bmp,
        Some("tiff") => ImageFormat::Tiff,
        Some("ico") => ImageFormat::Ico,
        Some("webp") => ImageFormat::WebP,
        _ => return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported file extension"))),
    };

    // Write the image to the specified output format
    let mut output_file = BufWriter::new(File::create(output_file)?);
    img.write_to(&mut output_file, output_format).expect("Failed to write image in the specified format");

    bar.inc(bmp_body.len() as u64);
    bar.finish_with_message("Conversion complete");

    Ok(())
}
