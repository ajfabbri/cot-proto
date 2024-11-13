use quick_xml::Reader;

/// Support for detecting common variants of CoT messages used by TAK, and parsing them into
/// strongly-typed structs.
use crate::{
    detail::{parse, CotUnparsedDetail},
    Error,
};

/// An enum of expected message types.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TakCotType {
    GeoFence,
    Marker,
    RangeBearing,
    Route,
    Shape,
    Other,
}

/// Result of parsing a TAK CoT message and attempting to detect a message type.
///
/// We don't actually parse the `<detail>` section, since it is so dynamic in practice.
/// Thus, callers are responsible for further parsing of that section, for example using
/// [`quick_xml::Reader`].
pub struct TakCotMessage {
    pub cot_type: TakCotType,
    pub cot_msg: CotUnparsedDetail,
}

/// Use a heuristic to detect the type of TAK XML CoT message contained in supplied text.
///
/// Warning: This is based on known example messages from ATAK repository.
///   To get the raw type string, use [`parse_cot_msg_type()`].
pub fn detect_tak_cot_type(input: &str) -> Result<TakCotMessage, Error> {
    let cot_msg = parse(input)?;
    // List of tuples of <string to search for> -> <implied message type if found>
    // Order matters. Separated into base event `type` values and detail section values to search
    // for.
    let type_tokens = [
        ("u-r-b-", TakCotType::RangeBearing),
        ("u-rb-", TakCotType::RangeBearing),
        ("b-m-r", TakCotType::Route),
        ("u-d-", TakCotType::Shape),
    ];
    let detail_tokens = [
        ("__geofence", TakCotType::GeoFence),
        ("usericon", TakCotType::Marker),
    ];
    // First, search detail section for clues
    for line in &cot_msg.detail {
        for (search, msg_type) in detail_tokens.iter() {
            if line.contains(search) {
                return Ok(TakCotMessage {
                    cot_type: *msg_type,
                    cot_msg,
                });
            }
        }
    }
    // Next check the base event type
    for (search, msg_type) in type_tokens.iter() {
        if cot_msg.cot_type.contains(search) {
            return Ok(TakCotMessage {
                cot_type: *msg_type,
                cot_msg,
            });
        }
    }
    Ok(TakCotMessage {
        cot_type: TakCotType::Other,
        cot_msg,
    })
}

/// Parse `type` attribute from a CoT message XML string.
pub fn parse_cot_msg_type(text: &str) -> Result<String, Error> {
    match xml_first_element_w_attr(text, "event", "type") {
        Ok(Some(val)) => Ok(val),
        _ => Err(Error::BadField("No element 'event' with attribute 'type'")),
    }
}

/// XML parsing convenience
pub fn xml_first_element_w_attr(
    text: &str,
    elt_name: &str,
    attr_name: &str,
) -> Result<Option<String>, Error> {
    let mut reader = Reader::from_str(text);
    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event()? {
            // Parse attribute `type` in the `event` element.
            quick_xml::events::Event::Start(ref e) => {
                if e.name().into_inner() == elt_name.as_bytes() {
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.into_inner() == attr_name.as_bytes() {
                            return Ok(Some(String::from_utf8_lossy(&attr.value).to_string()));
                        }
                    }
                }
            }
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }
    Ok(None)
}

#[cfg(test)]
mod test {
    use crate::tak::test::get_xml_examples;

    use super::{detect_tak_cot_type, TakCotType};

    #[test]
    fn test_tak_cot_detect() {
        let examples = get_xml_examples().unwrap();
        for res in examples {
            let (filename, cot_xml) = res.unwrap();
            let cot = detect_tak_cot_type(&cot_xml).unwrap();
            if filename.starts_with("geo-fence") {
                assert_type(&filename, cot.cot_type, TakCotType::GeoFence);
            } else if filename.starts_with("marker-") {
                assert_type(&filename, cot.cot_type, TakCotType::Marker);
            } else if filename.starts_with("range-bearing") {
                assert_type(&filename, cot.cot_type, TakCotType::RangeBearing);
            } else if filename.starts_with("route") {
                assert_type(&filename, cot.cot_type, TakCotType::Route);
            } else if filename.starts_with("shape-") {
                assert_type(&filename, cot.cot_type, TakCotType::Shape);
            }
        }
    }

    fn assert_type(filename: &str, actual: TakCotType, expected: TakCotType) {
        assert_eq!(
            actual, expected,
            "{}: expected {:?}, got {:?}",
            filename, expected, actual
        );
    }
}
