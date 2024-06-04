use std::fs::File;
use std::io::{BufWriter, Write};

pub fn convert_to_sbr(input_file: &str, output_file: &str, header_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_file)?.to_rgb8();
    let (width, height) = img.dimensions();

    let mut r = Vec::with_capacity((width * height) as usize);
    let mut g = Vec::with_capacity((width * height) as usize);
    let mut b = Vec::with_capacity((width * height) as usize);

    for pixel in img.pixels() {
        r.push(pixel[0]);
        g.push(pixel[1]);
        b.push(pixel[2]);
    }

    let mut f = BufWriter::new(File::create(output_file)?);
    f.write_all(&r)?;
    f.write_all(&g)?;
    f.write_all(&b)?;

    let mut hf = BufWriter::new(File::create(header_file)?);
    writeln!(hf, "{},{}", width, height)?;

    Ok(())
}
