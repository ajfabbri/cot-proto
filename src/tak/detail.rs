use crate::base::serialize_date;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::base::deserialize_date;

/// Type definitions for CoT detail sections for TAK messages.
///
/// Limited message types supported so far.

/// `<detail>` section for a Marker message, with reasonable defaults to put a dot on a map (i.e.
/// when sent to TAK).
/// Note: ATAK's "Marker*.xsd" schemas don't list these elements as optional,
/// (i.e. missing `maxOccurs="0"`) but I was told by a dev that many are optional.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TakMarkerDetail {
    pub status: Status,
    pub link: Option<Link>,
    pub contact: Contact,
    pub remarks: Option<Remarks>,
    pub color: Option<Color>,
    pub precisionlocation: PrecisionLocation,
    pub usericon: Option<UserIcon>,
}

// TODO move these common definitions
#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    #[serde(rename = "@readiness")]
    pub readiness: bool,
}
impl Default for Status {
    fn default() -> Self {
        Status { readiness: true }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "@uid")]
    uid: String,
    #[serde(
        rename = "@production_time",
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub production_time: DateTime<Utc>,
    #[serde(rename = "@type")]
    pub cot_type: String,
    #[serde(rename = "@parent_callsign")]
    pub parent_callsign: String,
    #[serde(rename = "@relation")]
    pub relation: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    #[serde(rename = "@callsign")]
    pub callsign: String,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    pub endpoint: Option<String>,
    pub phone: Option<u32>,
    #[serde(rename = "xmppUsername")]
    pub xmpp_username: Option<String>,
}
impl Default for Contact {
    fn default() -> Self {
        Contact {
            callsign: "???".to_string(),
            email_address: None,
            endpoint: None,
            phone: None,
            xmpp_username: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Remarks {
    // TODO is is probably not right
    #[serde(rename = "$value")]
    pub source: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    #[serde(rename = "@argb")]
    pub argb: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrecisionLocation {
    #[serde(rename = "@altsrc")]
    pub altsrc: String,
    #[serde(rename = "@geopointsrc")]
    pub geopointsrc: Option<String>,
    #[serde(rename = "@PRECISE_IMAGE_FILE")]
    pub pi_file: Option<String>,
    #[serde(rename = "@PRECISE_IMAGE_FILE_X")]
    pub pi_file_x: Option<String>,
    #[serde(rename = "@PRECISE_IMAGE_FILE_Y")]
    pub pi_file_y: Option<String>,
}

impl Default for PrecisionLocation {
    fn default() -> Self {
        Self {
            altsrc: "???".to_string(),
            geopointsrc: None,
            pi_file: None,
            pi_file_x: None,
            pi_file_y: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIcon {
    #[serde(rename = "@iconsetpath")]
    iconsetpath: String,
}

#[cfg(test)]
mod test {
    use crate::base::Cot;

    use super::*;
    #[test]
    fn test_deserialize_tak_marker() {
        let xml_path = format!(
            "{}/src/tak/examples/marker-2525.cot",
            env!("CARGO_MANIFEST_DIR")
        );
        let xml_text = std::fs::read_to_string(&xml_path).unwrap();
        let marker: Cot<TakMarkerDetail> = quick_xml::de::from_str(&xml_text).unwrap();
        assert_eq!(marker.version, "2.0");
    }
}
