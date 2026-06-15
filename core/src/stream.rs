use crate::shared_strings;
use crate::sheet_parser::{CellValue, SheetParser};
use crate::zip_reader::XlsxZip;
use crate::workbook;
use crate::relationships;
use std::fs::File;
use std::path::Path;

pub struct XlsxStream {
    sheet_xml: Vec<u8>,
    sst: Vec<String>,
}

impl XlsxStream {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        Self::open_sheet(path, None)
    }

    pub fn open_sheet(path: impl AsRef<Path>, sheet_name: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut zip = XlsxZip::new(file)?;

        let sst = if zip.has_entry("xl/sharedStrings.xml") {
            let raw = zip.read_entry("xl/sharedStrings.xml")?;
            shared_strings::parse(&raw)?
        } else {
            Vec::new()
        };

        let sheet_path = if let Some(name) = sheet_name {
            // Parse workbook.xml to find sheet by name
            let workbook_xml = zip.read_entry("xl/workbook.xml")?;
            let sheets = workbook::parse_workbook(&workbook_xml)?;

            let sheet = sheets
                .iter()
                .find(|s| s.name == name)
                .ok_or_else(|| format!("Sheet '{}' not found", name))?;

            // Parse workbook.xml.rels to map rel_id to file path
            let rels_xml = zip.read_entry("xl/_rels/workbook.xml.rels")?;
            let rels = relationships::parse_relationships(&rels_xml)?;

            let target = rels
                .get(&sheet.rel_id)
                .ok_or_else(|| format!("Relationship '{}' not found", sheet.rel_id))?;

            // Target paths may start with /xl/ or be relative; normalize them
            if target.starts_with('/') {
                target.trim_start_matches('/').to_string()
            } else {
                format!("xl/{}", target)
            }
        } else {
            // Default to first sheet
            "xl/worksheets/sheet1.xml".to_string()
        };

        let sheet_xml = zip.read_entry(&sheet_path)?;

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
