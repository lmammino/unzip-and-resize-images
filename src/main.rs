use clap::{command, Parser};
use image::io::Reader as ImageReader;
use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

mod zip_iter;

/// Program to read images from a zip file and resize them to a square
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the zip file to open
    #[arg(short, long)]
    filename: String,

    /// Square size of the final images
    #[arg(short, long, default_value_t = 400)]
    size: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.filename)?;
    let mut zip = zip::ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        if file.name().starts_with("__MACOSX")
            || file.name().ends_with('/')
            || !file.name().ends_with(".jpg")
        {
            continue;
        }

        println!("Filename: {}", file.name());

        let mut bytes: Vec<u8> = Vec::with_capacity(usize::try_from(file.size())?);
        std::io::copy(&mut file, &mut bytes)?;

        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode();

        let img = match img {
            Ok(i) => i,
            Err(err) => {
                println!("Error: {:?}", err);
                continue;
            }
        };

        let scaled_img =
            img.resize_to_fill(args.size, args.size, image::imageops::FilterType::Lanczos3);
        let name = Path::new(file.name());

        // TODO: in the future add support for keeping the image structure
        let mut dest_file = File::create(format!(
            "output/{}.jpg",
            name.file_stem()
                .ok_or("invalid file stem")?
                .to_str()
                .ok_or("invalid file name encoding")?
        ))?;

        scaled_img.write_to(&mut dest_file, image::ImageOutputFormat::Jpeg(100))?;
    }

    Ok(())
}
