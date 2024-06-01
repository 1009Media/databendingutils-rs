mod tosbr;
mod fromsbr;
mod tohbmp;
mod fromhbmp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ascii_logo = r#"
██████╗  █████╗ ████████╗ █████╗ ██████╗ ███████╗███╗   ██╗██████╗ ██╗███╗   ██╗ ██████╗ ██╗   ██╗████████╗██╗██╗     ███████╗      ██████╗ ███████╗
██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗██╔══██╗██╔════╝████╗  ██║██╔══██╗██║████╗  ██║██╔════╝ ██║   ██║╚══██╔══╝██║██║     ██╔════╝      ██╔══██╗██╔════╝
██║  ██║███████║   ██║   ███████║██████╔╝█████╗  ██╔██╗ ██║██║  ██║██║██╔██╗ ██║██║  ███╗██║   ██║   ██║   ██║██║     ███████╗█████╗██████╔╝███████╗
██║  ██║██╔══██║   ██║   ██╔══██║██╔══██╗██╔══╝  ██║╚██╗██║██║  ██║██║██║╚██╗██║██║   ██║██║   ██║   ██║   ██║██║     ╚════██║╚════╝██╔══██╗╚════██║
██████╔╝██║  ██║   ██║   ██║  ██║██████╔╝███████╗██║ ╚████║██████╔╝██║██║ ╚████║╚██████╔╝╚██████╔╝   ██║   ██║███████╗███████║      ██║  ██║███████║
╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═════╝ ╚═╝╚═╝  ╚═══╝ ╚═════╝  ╚═════╝    ╚═╝   ╚═╝╚══════╝╚══════╝      ╚═╝  ╚═╝╚══════╝"#;

    println!("{}", ascii_logo);
    println!("1.0.1\n");
    loop {
        println!("Choose an operation:");
        println!("--------------------\n");
        println!("PRE DATABENDING");
        println!("1. Convert image to SBR");
        println!("2. Convert image to headless BMP\n");
        println!("--------------------\n");
        println!("POST DATABENDING");
        println!("3. Convert SBR to image");
        println!("4. Convert headless BMP to image\n");
        println!("--------------------\n");
        println!("OTHER");
        println!("0. Exit");
        println!("98. How to databend");
        println!("99. SBR Info\n");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "0" => {
                println!("Exiting program.");
                break;
            }
            "1" => {
                println!("Convert an input image to SBR.\n");

                let mut input_file = String::new();
                println!("File path to input image: ");
                std::io::stdin().read_line(&mut input_file)?;
                let input_file = input_file.trim();

                let mut output_file = String::new();
                println!("File path to output SBR: ");
                std::io::stdin().read_line(&mut output_file)?;
                let output_file = output_file.trim();

                let mut header_file = String::new();
                println!("File path to output header file: ");
                println!("Recommended to save in same folder as SBR");
                std::io::stdin().read_line(&mut header_file)?;
                let header_file = header_file.trim();

                tosbr::convert_to_sbr(input_file, output_file, header_file)?;
                println!("Conversion complete, exiting program.");
                break;
            }
            "2" => {
                println!("Convert an input image to headless BMP.\n");

                let mut input_file = String::new();
                println!("File path to input image: ");
                std::io::stdin().read_line(&mut input_file)?;
                let input_file = input_file.trim();

                let mut output_file = String::new();
                println!("File path to output headless BMP: ");
                std::io::stdin().read_line(&mut output_file)?;
                let output_file = output_file.trim();

                let mut header_file = String::new();
                println!("File path to output header file: ");
                println!("Recommended to save in same folder as headless BMP");
                std::io::stdin().read_line(&mut header_file)?;
                let header_file = header_file.trim();

                tohbmp::convert_to_hbmp(input_file, output_file, header_file)?;
                println!("Conversion complete, exiting program.");
                break;
            }
            "3" => {
                println!("Convert an SBR to a common image format.\n");

                let mut input_file = String::new();
                println!("File path to input SBR: ");
                std::io::stdin().read_line(&mut input_file)?;
                let input_file = input_file.trim();

                let mut header_file = String::new();
                println!("File path to input header file: ");
                std::io::stdin().read_line(&mut header_file)?;
                let header_file = header_file.trim();

                let mut output_file = String::new();
                println!("File path to output image: ");
                std::io::stdin().read_line(&mut output_file)?;
                let output_file = output_file.trim();

                fromsbr::convert_from_sbr(input_file, header_file, output_file)?;
                println!("Conversion complete, exiting program.");
                break;
            }
            "4" => {
                println!("Convert a headless BMP to a common image format.\n");

                let mut input_file = String::new();
                println!("File path to input headless BMP: ");
                std::io::stdin().read_line(&mut input_file)?;
                let input_file = input_file.trim();

                let mut header_file = String::new();
                println!("File path to input header file: ");
                std::io::stdin().read_line(&mut header_file)?;
                let header_file = header_file.trim();

                let mut output_file = String::new();
                println!("File path to output image: ");
                std::io::stdin().read_line(&mut output_file)?;
                let output_file = output_file.trim();

                fromhbmp::convert_from_hbmp(input_file, header_file, output_file)?;
                println!("Conversion complete, exiting program.");
                break;
            }
            "98" => {
                println!("Here's how to databend using this program:\n");
                
                println!("Step 1. Ensure Audacity is installed on your system.");
                println!("(This is not required, there are other ways to databend, however this is an easy way to get into it.)\n");
                
                println!("Step 2. Use this program to convert an image you want to databend to headless BMP, or SBR.\n");
                
                println!("Step 3. Open Audacity, go to File>Import>Raw Data, and double click the headless BMP or SBR file.\n");
                
                println!("Step 4. Set encoding to U-Law, you can leave the other settings on their defaults. Now click import.");
                
                println!("Step 5. You should see a waveform in Audacity, DO NOT PRESS PLAY. Apply audio effects to segments of the waveform, or to the entire thing, to databend the image.");
                println!("Note if you are using SBR: You should see three distinct segments in the waveform, this is the data for the red, green, and blue pixels. You can databend these separately.\n");
                
                println!("Step 6. Once you're done databending the image, you can go to File>Export Audio, choose the folder you wish to save it in, set Format to 'Other uncompressed files', Header to 'RAW (header-less)', and encoding to 'U-Law'. Click Export.\n");
                
                println!("Step 7. Once your databent image has exported, you can delete the original SBR or headless BMP, NOT THE HEADER, and rename the '.raw' file so the extnsion matches what the original SBR or headless BMP extension was.");
                
                println!("Step 8. Now you can use this program to convert your databent SBR or headless BMP back into a common image format.\n");
                
                println!("That's it, you're done.\n");
            }
            "99" => {
                println!("SBR is a custom image format I created for the purpose of databending the red, green, and blue pixels of an image separately.");
                println!("I know this is possible with TIFF, however the only way I know of to export TIFF images with an RRGGBB pixel layout is in Photoshop.");
                println!("This is not ideal because Photoshop is a paid, closed-source program that is unavailable on Linux, locking many people out of this form of databending.");
                println!("Additionally, the TIFF format is a pain to work with in terms of separating the header from the pixel data, and later reattaching it.\n");
                
                println!("The specifications of SBR are relatively straightforward:");
                println!("8 bit colour");
                println!("No alpha channel");
                println!("Header stored separately from the pixel data");
                println!("Pixel data stored as RRGGBB instead of the more common RGBRGB\n");
                
                println!("That last specification is the reason for SBR's name. Separate Blocks of Red (Green and Blue).");
                println!("Dogshit acronym I know. At least it's not recursive.\n");
            }
            _ => {
                println!("Invalid choice.\n");
            }
        }
    }

    Ok(())
}
