pub mod create;
pub mod detail;
pub mod detect;

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
            let (_filename, cot_xml) = res.unwrap();
            let _cot = parse(&cot_xml).unwrap();
        }
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

    pub fn get_xml_examples() -> Result<CotExamples, Error> {
        let examples_path = format!("{}/src/tak/examples", env!("CARGO_MANIFEST_DIR"));
        let examples = CotExamples::new(examples_path).unwrap();
        assert!(examples.len() > 0);
        Ok(examples)
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
