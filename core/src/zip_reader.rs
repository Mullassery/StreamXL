use std::io::{Read, Seek};
use zip::ZipArchive;

pub struct XlsxZip<R: Read + Seek> {
    archive: ZipArchive<R>,
}

impl<R: Read + Seek> XlsxZip<R> {
    pub fn new(reader: R) -> Result<Self, zip::result::ZipError> {
        Ok(Self {
            archive: ZipArchive::new(reader)?,
        })
    }

    pub fn read_entry(&mut self, name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut entry = self.archive.by_name(name)?;
        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn has_entry(&mut self, name: &str) -> bool {
        self.archive.by_name(name).is_ok()
    }
}
