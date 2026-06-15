use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

pub fn parse_relationships(xml: &[u8]) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);

    let mut relationships = HashMap::new();

    loop {
        match reader.read_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let elem_name = e.name();
                let name_str = String::from_utf8_lossy(elem_name.as_ref());

                if name_str.ends_with("Relationship") {
                    let mut id = String::new();
                    let mut target = String::new();

                    for attr in e.attributes() {
                        match attr {
                            Ok(a) => {
                                let key_str = String::from_utf8_lossy(a.key.as_ref());
                                let value = String::from_utf8_lossy(&a.value).into_owned();

                                if key_str == "Id" {
                                    id = value;
                                } else if key_str == "Target" {
                                    target = value;
                                }
                            }
                            Err(_) => continue,
                        }
                    }

                    if !id.is_empty() && !target.is_empty() {
                        relationships.insert(id, target);
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(relationships)
}
