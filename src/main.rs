use image::io::Reader as ImageReader;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: VecDeque<String> = env::args().skip(1).collect();
    let filename = args.pop_front().expect("No filename provided");
    let image_size: u32 = args.pop_front().unwrap_or("400".to_string()).parse()?;
    let file = File::open(filename)?;
    let mut zip = zip::ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());

        let mut bytes: Vec<u8> = Vec::with_capacity(usize::try_from(file.size())?);
        std::io::copy(&mut file, &mut bytes)?;

        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;

        let scaled_img = img.resize_to_fill(
            image_size,
            image_size,
            image::imageops::FilterType::Lanczos3,
        );
        let name = Path::new(file.name());

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
