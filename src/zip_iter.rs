use generator::{done, Generator, Gn};
use std::io::{Read, Seek};
use zip::{read::ZipFile, ZipArchive};

pub fn zip_iter<'a, R: Read + Seek>(zip: ZipArchive<R>) -> impl Iterator<Item = ZipFile<'a>> {
    // create a generator that iterates over all the files in the zip archive

    let a = Gn::new(move || {
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            yield file;
        }
        done!()
    });

    a
}
