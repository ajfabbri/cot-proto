//! Simple Cursor on Target (CoT) (de)serialization library.
//!
//! Provides struct definitions and utilities which use
//! [quick_xml](https://github.com/tafia/quick-xml) to handle conversion to/from XML text.
//!
//! The CoT base schema is represented by the [base::Cot] struct. You can either provide your own
//! type for the detail section of the CoT message, or you can use [detail::parse()] to skip
//! parsing the section and instead get raw XML text. What we really want is a dynamic type,
//! analogous to [serde_json::Value](https://docs.rs/serde_json/latest/serde_json/enum.Value.html),
//! but I haven't found anything similar for XML.
//!

use thiserror::Error;
pub mod base;
pub mod detail;
pub mod examples;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Xml(#[from] quick_xml::errors::Error),
    #[error(transparent)]
    De(#[from] quick_xml::de::DeError),
}

#[cfg(test)]
mod test {
    use quick_xml::events::Event;

    use crate::examples::COT_TRACK_EXAMPLE;

    #[test]
    fn test_fastxml_extract() {
        let mut reader = quick_xml::Reader::from_reader(COT_TRACK_EXAMPLE.as_bytes());
        reader.config_mut().trim_text(true);
        let mut detail: Vec<String> = vec![];
        let mut is_detail = false;
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"detail" {
                        is_detail = true;
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    if is_detail {
                        detail.push(String::from_utf8_lossy(e).to_string());
                    }
                }
                Ok(Event::Text(_e)) => {
                    //println!("Event::Text: {:?}", e);
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"detail" {
                        is_detail = false;
                    }
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => println!("Event: {:?}", reader.read_event()),
            }
        }
    }
}
