use serde::{Deserialize, Serialize};

/// Base schema structure and ser/de.

// See References section in README.md
pub const COT_BASE_EXAMPLE: &str = r#"
<?xml version='1.0' standalone='yes'?>
<event version="2.0"
 uid="J-01334"
 type="a-h-A-M-F-U-M"
 time="2005-04-05T11:43:38.07Z"
 start="2005-04-05T11:43:38.07Z"
 stale="2005-04-05T11:45:38.07Z" >
 <detail>
 </detail>
 <point lat="30.0090027" lon="-85.9578735" ce="45.3"
 hae="-42.6" le="99.5" />
</event>
"#;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "event")]
pub struct Cot<D> {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@uid")]
    pub uid: String,
    #[serde(rename = "@type")]
    pub cot_type: String,
    #[serde(rename = "@time")]
    pub time: String,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@stale")]
    pub stale: String,
    #[serde(rename = "detail")]
    pub detail: D,
    #[serde(rename = "point")]
    pub point: Point,
}

pub type CotBase = Cot<NoDetail>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NoDetail {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Point {
    #[serde(rename = "@lat")]
    pub lat: f64,
    #[serde(rename = "@lon")]
    pub lon: f64,
    #[serde(rename = "@ce")]
    pub ce: f32,
    #[serde(rename = "@hae")]
    pub hae: f32,
    #[serde(rename = "@le")]
    pub le: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_serde() {
        // Create two Cot objects, one from example string and another from a round trip from that
        // to a string and back again. Validate values match.
        let cot0: CotBase = quick_xml::de::from_str(COT_BASE_EXAMPLE).unwrap();
        let cot_str = quick_xml::se::to_string(&cot0).unwrap();
        let cot1: CotBase = quick_xml::de::from_str(&cot_str).unwrap();
        assert_eq!(cot0, cot1);
    }
}
