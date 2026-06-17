use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub enum WriteCell {
    Str(String),
    Num(f64),
    Bool(bool),
    Empty,
}

pub struct XlsxWriter {
    output_path: PathBuf,
    sheet_buf: Vec<u8>,
    sst: Vec<String>,
    sst_index: HashMap<String, usize>,
}

impl XlsxWriter {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let mut sheet_buf = Vec::with_capacity(64 * 1024);
        sheet_buf.extend_from_slice(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
\n<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">\
\n<sheetData>\n",
        );
        Self {
            output_path: path.as_ref().to_path_buf(),
            sheet_buf,
            sst: Vec::new(),
            sst_index: HashMap::new(),
        }
    }

    pub fn write_row(&mut self, cells: &[WriteCell]) {
        self.sheet_buf.extend_from_slice(b"<row>");
        for cell in cells {
            match cell {
                WriteCell::Str(s) => {
                    let idx = match self.sst_index.get(s) {
                        Some(&i) => i,
                        None => {
                            let i = self.sst.len();
                            self.sst.push(s.clone());
                            self.sst_index.insert(s.clone(), i);
                            i
                        }
                    };
                    write!(self.sheet_buf, "<c t=\"s\"><v>{idx}</v></c>").unwrap();
                }
                WriteCell::Num(n) => {
                    write!(self.sheet_buf, "<c><v>{n}</v></c>").unwrap();
                }
                WriteCell::Bool(b) => {
                    let v = if *b { 1u8 } else { 0u8 };
                    write!(self.sheet_buf, "<c t=\"b\"><v>{v}</v></c>").unwrap();
                }
                WriteCell::Empty => {
                    self.sheet_buf.extend_from_slice(b"<c/>");
                }
            }
        }
        self.sheet_buf.extend_from_slice(b"</row>\n");
    }

    pub fn finish(mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.sheet_buf.extend_from_slice(b"</sheetData>\n</worksheet>");

        let file = File::create(&self.output_path)?;
        let mut zip = ZipWriter::new(file);
        let opts = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        let has_sst = !self.sst.is_empty();

        zip.start_file("[Content_Types].xml", opts)?;
        zip.write_all(content_types_xml(has_sst).as_bytes())?;

        zip.start_file("_rels/.rels", opts)?;
        zip.write_all(RELS_XML)?;

        zip.start_file("xl/workbook.xml", opts)?;
        zip.write_all(WORKBOOK_XML)?;

        zip.start_file("xl/_rels/workbook.xml.rels", opts)?;
        zip.write_all(workbook_rels_xml(has_sst).as_bytes())?;

        zip.start_file("xl/styles.xml", opts)?;
        zip.write_all(STYLES_XML)?;

        if has_sst {
            zip.start_file("xl/sharedStrings.xml", opts)?;
            zip.write_all(build_sst(&self.sst).as_bytes())?;
        }

        zip.start_file("xl/worksheets/sheet1.xml", opts)?;
        zip.write_all(&self.sheet_buf)?;

        zip.finish()?;
        Ok(())
    }
}

fn content_types_xml(has_sst: bool) -> String {
    let sst_part = if has_sst {
        "<Override PartName=\"/xl/sharedStrings.xml\" \
ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml\"/>"
    } else {
        ""
    };
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
\n<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\
\n<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
\n<Default Extension=\"xml\" ContentType=\"application/xml\"/>\
\n<Override PartName=\"/xl/workbook.xml\" \
ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/>\
\n<Override PartName=\"/xl/worksheets/sheet1.xml\" \
ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/>\
\n<Override PartName=\"/xl/styles.xml\" \
ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml\"/>\
\n{sst_part}\
\n</Types>"
    )
}

fn workbook_rels_xml(has_sst: bool) -> String {
    let sst_rel = if has_sst {
        "<Relationship Id=\"rId2\" \
Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings\" \
Target=\"sharedStrings.xml\"/>"
    } else {
        ""
    };
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\
\n<Relationship Id=\"rId1\" \
Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet\" \
Target=\"worksheets/sheet1.xml\"/>\
\n<Relationship Id=\"rId3\" \
Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles\" \
Target=\"styles.xml\"/>\
\n{sst_rel}\
\n</Relationships>"
    )
}

fn build_sst(strings: &[String]) -> String {
    let count = strings.len();
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
\n<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" \
count=\"{count}\" uniqueCount=\"{count}\">\n"
    );
    for s in strings {
        let escaped = s
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;");
        out.push_str("<si><t>");
        out.push_str(&escaped);
        out.push_str("</t></si>\n");
    }
    out.push_str("</sst>");
    out
}

const RELS_XML: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\
\n<Relationship Id=\"rId1\" \
Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" \
Target=\"xl/workbook.xml\"/>\
\n</Relationships>";

const WORKBOOK_XML: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
\n<workbook xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" \
xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\">\
\n<sheets>\
\n<sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/>\
\n</sheets>\
\n</workbook>";

const STYLES_XML: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
\n<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">\
\n<fonts count=\"1\"><font><sz val=\"11\"/><name val=\"Calibri\"/></font></fonts>\
\n<fills count=\"2\">\
\n<fill><patternFill patternType=\"none\"/></fill>\
\n<fill><patternFill patternType=\"gray125\"/></fill>\
\n</fills>\
\n<borders count=\"1\"><border><left/><right/><top/><bottom/><diagonal/></border></borders>\
\n<cellStyleXfs count=\"1\"><xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\"/></cellStyleXfs>\
\n<cellXfs count=\"1\"><xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\" xfId=\"0\"/></cellXfs>\
\n</styleSheet>";
