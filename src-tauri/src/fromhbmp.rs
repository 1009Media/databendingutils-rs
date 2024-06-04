use std::fs::File;
use std::io::{self, Read, BufReader, BufWriter};
use image::{ImageFormat, load_from_memory};
use std::path::Path;

pub fn convert_from_hbmp(
    input_file: &str,
    header_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut header_file = BufReader::new(File::open(header_file)?);
    let mut header = Vec::new();
    header_file.read_to_end(&mut header)?;

    let mut input_file = BufReader::new(File::open(input_file)?);
    let mut bmp_body = Vec::new();
    input_file.read_to_end(&mut bmp_body)?;

    let mut complete_bmp_data = Vec::new();
    complete_bmp_data.extend_from_slice(&header);
    complete_bmp_data.extend_from_slice(&bmp_body);

    let img = load_from_memory(&complete_bmp_data).expect("Failed to load BMP image from memory");

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

    let mut output_file = BufWriter::new(File::create(output_file)?);
    img.write_to(&mut output_file, output_format).expect("Failed to write image in the specified format");

    Ok(())
}
