use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Clone)]
pub enum CellValue {
    String(String),
    Number(f64),
    Bool(bool),
    Empty,
}

/// Streams rows from sheet XML, resolving shared string indices via `sst`.
pub struct SheetParser<'a> {
    reader: Reader<&'a [u8]>,
    sst: &'a [String],
}

impl<'a> SheetParser<'a> {
    pub fn new(xml: &'a [u8], sst: &'a [String]) -> Self {
        let mut reader = Reader::from_reader(xml);
        reader.config_mut().trim_text(true);
        Self { reader, sst }
    }

    pub fn next_row(&mut self) -> Result<Option<Vec<CellValue>>, Box<dyn std::error::Error>> {
        let mut row: Option<Vec<CellValue>> = None;
        let mut cell_type = String::new();
        let mut cell_value = String::new();
        let mut in_v = false;   // inside <v> (numeric / shared-string index)
        let mut in_t = false;   // inside <is><t> (inlineStr text)

        loop {
            match self.reader.read_event()? {
                Event::Start(ref e) => match e.name().as_ref() {
                    b"row" => row = Some(Vec::new()),
                    b"c" => {
                        cell_type = e
                            .attributes()
                            .flatten()
                            .find(|a| a.key.as_ref() == b"t")
                            .map(|a| String::from_utf8_lossy(&a.value).into_owned())
                            .unwrap_or_else(|| "n".to_string());
                        cell_value.clear();
                    }
                    b"v" => in_v = true,
                    // inlineStr stores text in <is><t>...</t></is>
                    b"t" if cell_type == "inlineStr" => in_t = true,
                    _ => {}
                },
                Event::Text(ref e) if in_v || in_t => {
                    cell_value.push_str(&e.unescape()?);
                }
                // Self-closing empty cell: <c/> or <c r="A1"/>
                Event::Empty(ref e) if e.name().as_ref() == b"c" => {
                    if let Some(ref mut r) = row {
                        r.push(CellValue::Empty);
                    }
                }
                Event::End(ref e) => match e.name().as_ref() {
                    b"v" => in_v = false,
                    b"t" => in_t = false,
                    b"c" => {
                        if let Some(ref mut r) = row {
                            r.push(self.resolve_cell(&cell_type, &cell_value));
                        }
                    }
                    b"row" => {
                        if let Some(r) = row.take() {
                            return Ok(Some(r));
                        }
                    }
                    b"sheetData" => return Ok(None),
                    _ => {}
                },
                Event::Eof => return Ok(None),
                _ => {}
            }
        }
    }

    fn resolve_cell(&self, cell_type: &str, value: &str) -> CellValue {
        match cell_type {
            "s" => {
                let idx: usize = value.parse().unwrap_or(0);
                self.sst
                    .get(idx)
                    .map(|s| CellValue::String(s.clone()))
                    .unwrap_or(CellValue::Empty)
            }
            "inlineStr" => CellValue::String(value.to_string()),
            "b" => CellValue::Bool(value == "1"),
            _ if value.is_empty() => CellValue::Empty,
            _ => value
                .parse::<f64>()
                .map(CellValue::Number)
                .unwrap_or_else(|_| CellValue::String(value.to_string())),
        }
    }
}
