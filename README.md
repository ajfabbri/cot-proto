[![Build Status]][Actions] [![Latest Version]][crates.io] [![Documentation]][docs.rs]

[Build Status]: https://github.com/ajfabbri/cot-proto/actions/workflows/ci.yml/badge.svg
[Actions]: https://github.com/ajfabbri/cot-proto/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/cot_proto.svg
[crates.io]: https://crates.io/crates/cot\_proto
[Documentation]: https://docs.rs/cot_proto/badge.svg
[docs.rs]: https://docs.rs/cot_proto

# Cursor on Target (COT) Protocol

This library aims to provide a simple way to produce (serialize) and consume
(deserialize) Cursor on Target (CoT) messages.

### Initial goals:
- Support for base CoT schema
- Basic Rust structs for CoT messages, with serde support.

### TODOs
- [ ] better types for timestamps/dates. Currently just strings.
- [ ] Add more typed schemas for common detail contents (sub-schemas)

## References
The following sources were used to help develop this library:
1. Michael J. Kristan et al. November 2009 [Cursor on Target (CoT) Message Router User's Guide](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf).
2. Mitre Corporation, via CoTreceiver. Captured Nov 2024. [CoT Schema Definitions (XSD)](https://github.com/mdudel/CoTreceiver/tree/master/lib/xsd)
3. Android Tactical Assualt Kit (ATAK) CIV. Captured Nov 2024. [takcot/examples](https://github.com/deptofdefense/AndroidTacticalAssaultKit-CIV/tree/main/takcot/examples)
