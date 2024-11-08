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
/// We don't actually parse the `<detail>` section, since it is so dynamic in practice.
/// Thus, callers are responsible for further parsing of that section, for example using
/// [`quick_xml::Reader`].
pub struct TakCotMessage {
    pub cot_type: TakCotType,
    pub cot_msg: CotUnparsedDetail,
}

/// Use a heuristic to detect the type of TAK XML CoT message contained in supplied text.
/// Warning: This is based on known example messages from ATAK repository. Please do extended
/// integration testing and file issues for any bugs you find.
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

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use quick_xml::de::from_str;
    use serde::Serialize;
    use serde_json::Value;

    use crate::{detail::parse, Error};

    #[test]
    fn test_tak_cot_examples() {
        let examples = get_xml_examples().unwrap();
        for res in examples {
            let (filename, cot_xml) = res.unwrap();
            let cot = parse(&cot_xml).unwrap();
            println!("{filename}\n\t{:?}", cot);
        }
    }

    #[test]
    fn test_tak_cot_detect() {
        let examples = get_xml_examples().unwrap();
        for res in examples {
            let (filename, cot_xml) = res.unwrap();
            let cot = crate::tak::detect_tak_cot_type(&cot_xml).unwrap();
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

    /// You can use serde_json::Value for storing dynamic XML data, except for the issue with
    /// repeated elements being lost. This only retains the last element in a sequence with the
    /// same name. This can be addressed by implementing a custom quick_xml reader and using a
    /// HashMap of Vecs to store repeated elements. What a pain.
    #[test]
    fn test_tak_to_json() {
        let mut examples = get_xml_examples().unwrap();
        let (first_name, first) = examples.next().unwrap().unwrap();
        let json_val: Value = from_str(&first).unwrap();
        println!("{}", serde_json::to_string_pretty(&json_val).unwrap());
        #[derive(Debug, Serialize)]
        #[serde(rename = "event")]
        struct Root(Value);
        let root_val = Root(json_val);
        let xml1 = quick_xml::se::to_string(&root_val).unwrap();
        println!("----> {}", first_name);
        println!("initial xml:\n\t{}", &first);
        println!("after json round trip:\n\t{}", xml1);
    }

    fn get_xml_examples() -> Result<CotExamples, Error> {
        let examples_path = format!("{}/src/tak/examples", env!("CARGO_MANIFEST_DIR"));
        let examples = CotExamples::new(examples_path).unwrap();
        assert!(examples.len() > 0);
        Ok(examples)
    }
    use regex::Regex;

    use super::TakCotType;

    #[allow(dead_code)]
    fn trim_whitespace(xml: &str) -> String {
        let re = Regex::new(r">\s+<").unwrap();
        re.replace_all(xml, "><").to_string()
    }

    // Test helper to iterate over all example messages
    pub struct CotExamples {
        paths: Vec<PathBuf>,
    }
    impl CotExamples {
        pub fn new(examples_path: String) -> Result<Self, Error> {
            let paths = std::fs::read_dir(&examples_path)?
                .map(|r| r.unwrap().path())
                .filter(|p| p.extension().map(|ext| ext == "cot").unwrap_or(false))
                .collect();
            Ok(CotExamples { paths })
        }

        pub fn len(&self) -> usize {
            self.paths.len()
        }
    }

    impl Iterator for CotExamples {
        // Iterator yeilds (filename, contents) tuples
        type Item = Result<(String, String), Error>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.paths.is_empty() {
                return None;
            }
            let path = self.paths.remove(0);
            let basename = path.file_name().unwrap().to_string_lossy().to_string();
            let res: Result<String, Error> = std::fs::read_to_string(&path).map_err(|e| e.into());
            match res {
                Ok(text) => Some(Ok((basename, text))),
                Err(e) => Some(Err(e)),
            }
        }
    }
}
