use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Clone)]
pub struct SheetInfo {
    pub name: String,
    pub id: String,
    pub rel_id: String,
}

pub fn parse_workbook(xml: &[u8]) -> Result<Vec<SheetInfo>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);

    let mut sheets = Vec::new();

    loop {
        match reader.read_event()? {
            Event::Empty(e) => {
                let elem_name = e.name();
                let name_str = String::from_utf8_lossy(elem_name.as_ref());

                if name_str.ends_with("sheet") {
                    let mut sheet_name = String::new();
                    let mut sheet_id = String::new();
                    let mut rel_id = String::new();

                    for attr in e.attributes() {
                        match attr {
                            Ok(a) => {
                                let key_bytes = a.key.as_ref();
                                let value = String::from_utf8_lossy(&a.value).into_owned();

                                if key_bytes == b"name" {
                                    sheet_name = value;
                                } else if key_bytes == b"sheetId" {
                                    sheet_id = value;
                                } else if key_bytes == b"r:id" {
                                    rel_id = value;
                                }
                            }
                            Err(_) => continue,
                        }
                    }

                    if !sheet_name.is_empty() && !rel_id.is_empty() {
                        sheets.push(SheetInfo { name: sheet_name, id: sheet_id, rel_id });
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(sheets)
}
