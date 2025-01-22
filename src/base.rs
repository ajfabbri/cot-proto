use chrono::{DateTime, SecondsFormat, Utc};
use quick_xml::Reader;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Error;

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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "event")]
pub struct Cot<D> {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@uid")]
    pub uid: String,
    #[serde(rename = "@type")]
    pub cot_type: String,
    #[serde(
        rename = "@time",
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub time: DateTime<Utc>,
    #[serde(
        rename = "@start",
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub start: DateTime<Utc>,
    #[serde(
        rename = "@stale",
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub stale: DateTime<Utc>,
    #[serde(rename = "@how", skip_serializing_if = "Option::is_none")]
    pub how: Option<String>,
    #[serde(rename = "detail")]
    pub detail: D,
    #[serde(rename = "point")]
    pub point: Point,
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

pub(crate) fn serialize_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339_opts(SecondsFormat::Millis, true /* use Z for +00:00 */);
    serializer.serialize_str(&s)
}

pub(crate) fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map_err(DeError::custom)
        .map(|dt| dt.with_timezone(&Utc))
}

pub type CotBase = Cot<NoDetail>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NoDetail {}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

impl Point {
    pub fn north_pole() -> Self {
        Self {
            lat: 90.0,
            lon: 0.0,
            ce: 0.0,
            hae: 0.0,
            le: 0.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_serde_roundtrip() {
        // Create two Cot objects, one from example string and another from a round trip from that
        // to a string and back again. Validate values match.
        let cot0: CotBase = quick_xml::de::from_str(COT_BASE_EXAMPLE).unwrap();
        let cot_str = quick_xml::se::to_string(&cot0).unwrap();
        let cot1: CotBase = quick_xml::de::from_str(&cot_str).unwrap();
        assert_eq!(cot0, cot1);
    }
}
