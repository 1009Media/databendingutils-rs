use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom, Cursor};
use image::ImageFormat;

pub fn convert_to_hbmp(input_file: &str, output_file: &str, header_file: &str) -> io::Result<()> {
    let img = image::open(input_file).expect("Failed to open input image");

    let mut bmp_data = Cursor::new(Vec::new());
    img.write_to(&mut bmp_data, ImageFormat::Bmp).expect("Failed to write BMP data");

    let mut bmp_file = File::create(output_file)?;
    bmp_file.write_all(bmp_data.get_ref())?;

    let header_size = 54;
    let mut bmp_file = File::open(output_file)?;
    let mut header = vec![0; header_size];
    let mut bmp_body = Vec::new();

    bmp_file.read_exact(&mut header)?;
    bmp_file.seek(SeekFrom::Start(header_size as u64))?;
    bmp_file.read_to_end(&mut bmp_body)?;

    let mut header_file = File::create(header_file)?;
    header_file.write_all(&header)?;

    let mut bmp_file = File::create(output_file)?;
    bmp_file.write_all(&bmp_body)?;

    Ok(())
}
