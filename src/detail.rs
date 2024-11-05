use quick_xml::events::Event;

use crate::{
    base::{Cot, CotBase},
    Error,
};

/// A CoT message struct with unparsed \<detail\> section, which is captured as a raw
/// `Vec<String>`.
pub type CotUnparsedDetail = Cot<Vec<String>>;

impl From<CotBase> for CotUnparsedDetail {
    fn from(cot: CotBase) -> Self {
        CotUnparsedDetail {
            version: cot.version,
            uid: cot.uid,
            cot_type: cot.cot_type,
            time: cot.time,
            start: cot.start,
            stale: cot.stale,
            detail: vec![],
            point: cot.point,
        }
    }
}

/// Deserialize a UTF8 XML CoT message into a struct, but capture the `<detail>` section as an
/// unparsed `Vec<String>`.
/// If you want to parse the `<detail>` section into a strongly-typed struct, instead do this:
/// ```rust
/// # use cot_proto::base::Cot;
/// # use cot_proto::examples::COT_TRACK_EXAMPLE;
/// # #[derive(serde::Deserialize)]
/// # struct Foo {}
/// # let input_str = COT_TRACK_EXAMPLE;
/// let cot: Cot<Foo> = quick_xml::de::from_str(input_str).unwrap();
/// ```
/// where `Foo` is your known type struct for the detail section, which implements `Deserialize`.
pub fn parse(input: &str) -> Result<CotUnparsedDetail, Error> {
    let mut reader = quick_xml::Reader::from_str(input);
    reader.config_mut().trim_text(true);
    let detail = extract_detail(reader)?;
    let cot_base: CotBase = quick_xml::de::from_str(input)?;
    let mut cot: CotUnparsedDetail = cot_base.into();
    cot.detail = detail;
    Ok(cot)
}

/// Extract the `<detail>` section from a CoT message without trying to parse it into a concrete
/// type.
pub fn extract_detail(mut reader: quick_xml::reader::Reader<&[u8]>) -> Result<Vec<String>, Error> {
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
                    // XXX there should be a better way to get raw lines here?
                    detail.push(format!("<{}/>", String::from_utf8_lossy(e)));
                }
            }
            Ok(Event::Text(_e)) => {}
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"detail" {
                    is_detail = false;
                }
            }
            Err(e) => return Err(Error::Xml(e)),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    Ok(detail)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::examples::{
        COT_STRIKE_DETAIL_LINES, COT_STRIKE_EXAMPLE, COT_TRACK_DETAIL_LINES, COT_TRACK_EXAMPLE,
    };

    #[test]
    fn test_detail_parse_track2() {
        test_expected_detail(COT_TRACK_EXAMPLE, &COT_TRACK_DETAIL_LINES)
    }

    #[test]
    fn test_detail_parse_strike() {
        test_expected_detail(COT_STRIKE_EXAMPLE, &COT_STRIKE_DETAIL_LINES)
    }

    fn test_expected_detail(input: &str, expected_lines: &[&str]) {
        let cot = super::parse(input).unwrap();
        let mut expected_lines: HashSet<&str> = HashSet::from_iter(expected_lines.iter().cloned());
        for line in &cot.detail {
            let removed = expected_lines.remove(&line.as_str());
            if !removed {
                panic!(
                    "Unexpected line: {:?}\n  not in: {:?}",
                    line, expected_lines
                );
            }
        }
        assert_eq!(expected_lines.len(), 0);
    }
}
