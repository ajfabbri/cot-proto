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

pub const COT_TRACK_EXAMPLE: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<event version="2.0" uid="1228717" type="a-f-A-M-F-Q" how="m-g" time="2023-08-21T11:47:48.0Z" start="2023-08-21T12:47:02.283Z" stale="2023-08-21T12:57:07.283Z" qos="5-r-c">
<point ce="10.0" le="10.0" hae="1321.29992675781" lat="-23.14187321890312" lon="126.87965999741635"/>
<detail> <track course="0" speed="0" version="0.2"/>
<contact callsign="BLAMO-IDM1-3V"/>
<_flow-tags_ TAK-Server-ae386e25da33412635239519c6f0e1ae="2023-07-21T11:52:33Z"/>
</detail>
</event>
"#;

pub const COT_TRACK_DETAIL_LINES: [&str; 3] = [
    r#"<track course="0" speed="0" version="0.2"/>"#,
    r#"<contact callsign="BLAMO-IDM1-3V"/>"#,
    r#"<_flow-tags_ TAK-Server-ae386e25da33412635239519c6f0e1ae="2023-07-21T11:52:33Z"/>"#,
];

pub const COT_STRIKE_EXAMPLE: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<event version="2.0" type="t-k" uid="FAB.BOOT.a-h-G-U-C-I_MSN-01" time="2023-10-24T03:16:14.897441Z" start="2023-10-24T03:16:15.897441Z" stale="2023-10-24T03:18:14.897441Z" how="m-g" access="Unclassified" opex="e" qos="3-i-g">
  <point lat="-35.3414455365394" lon="149.9822337129055" hae="1.0" le="10.0" ce="5.0" />
  <detail>
    <takv platform="PyTAK" version="6.2.1-beta1" />
    <contact callsign="FAB_MSN-01" email="BOOT@usarmy.mil" />
    <__group name="Purple" role="Team Member" />
    <_tasking= TASK@FABNUM”=“MDN12345” TASK@ASSET=“YOURASSETUID12345” TASK@TGT=“THE TARGET OBJECT UID12345” />
    <remarks FAB_CCMA_BOOT="Use the force Luke. Object location is in point" time="2023-10- 24T03:16:14.985238Z" />
    <_flow-tags_ FAB_DESERT="2023-10-24T03:16:14.985238Z" FAB_CCMA_AFSATE="2023-10-24T03:16:14.985238Z" />
  </detail>
</event>
"#;

pub const COT_STRIKE_DETAIL_LINES: [&str; 6] = [
    r#"<takv platform="PyTAK" version="6.2.1-beta1" />"#,
    r#"<contact callsign="FAB_MSN-01" email="BOOT@usarmy.mil" />"#,
    r#"<__group name="Purple" role="Team Member" />"#,
    r#"<_tasking= TASK@FABNUM”=“MDN12345” TASK@ASSET=“YOURASSETUID12345” TASK@TGT=“THE TARGET OBJECT UID12345” />"#,
    r#"<remarks FAB_CCMA_BOOT="Use the force Luke. Object location is in point" time="2023-10- 24T03:16:14.985238Z" />"#,
    r#"<_flow-tags_ FAB_DESERT="2023-10-24T03:16:14.985238Z" FAB_CCMA_AFSATE="2023-10-24T03:16:14.985238Z" />"#,
];
