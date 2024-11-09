use chrono::Utc;

use crate::base::{Cot, Point};

use super::detail::TakMarkerDetail;

/// Support for creating TAK CoT messages with reasonable defaults for quickly getting integration
/// working.
///
/// Instead of providing builder APIs, we implement [`Default`] on different CoT variants: You'll
/// want to modify key fields like `point` with your real coordinates. For example:
/// ```rust
/// use cot_proto::base::{Cot, Point};
/// use cot_proto::tak::detail::TakMarkerDetail;
/// let mut cot = Cot::<TakMarkerDetail>::default();
/// cot.point.lat = 10.0;
/// cot.point.lon = 90.0;
/// let xml_text = quick_xml::se::to_string(&cot).unwrap();
/// ```

/// Default CoT type for marker messages.
pub const DEFAULT_COT_TYPE_MARKER: &str = "a-o-G";

/// TAK CoT Marker
impl Default for Cot<TakMarkerDetail> {
    fn default() -> Self {
        let now = Utc::now();
        let detail = TakMarkerDetail {
            ..Default::default()
        };
        Self {
            version: "2.0".to_string(),
            uid: uuid::Uuid::new_v4().to_string(),
            cot_type: DEFAULT_COT_TYPE_MARKER.to_string(),
            time: now,
            start: now,
            // now plus 1 day
            stale: now + chrono::Duration::days(1),
            how: Some("m-g".to_string()),
            detail,
            point: Point::north_pole(),
        }
    }
}
