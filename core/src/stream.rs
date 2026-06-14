use crate::shared_strings;
use crate::sheet_parser::{CellValue, SheetParser};
use crate::zip_reader::XlsxZip;
use std::fs::File;
use std::path::Path;

pub struct XlsxStream {
    sheet_xml: Vec<u8>,
    sst: Vec<String>,
}

impl XlsxStream {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut zip = XlsxZip::new(file)?;

        let sst = if zip.has_entry("xl/sharedStrings.xml") {
            let raw = zip.read_entry("xl/sharedStrings.xml")?;
            shared_strings::parse(&raw)?
        } else {
            Vec::new()
        };

        // Default to first sheet (xl/worksheets/sheet1.xml)
        let sheet_xml = zip.read_entry("xl/worksheets/sheet1.xml")?;

        Ok(Self { sheet_xml, sst })
    }

    pub fn rows(&self) -> RowIter<'_> {
        RowIter {
            parser: SheetParser::new(&self.sheet_xml, &self.sst),
            done: false,
        }
    }
}

pub struct RowIter<'a> {
    parser: SheetParser<'a>,
    done: bool,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Result<Vec<CellValue>, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.parser.next_row() {
            Ok(Some(row)) => Some(Ok(row)),
            Ok(None) => {
                self.done = true;
                None
            }
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }
}
