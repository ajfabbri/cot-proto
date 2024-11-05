# Cursor on Target (COT) Protocol

This library aims to provide a simple way to produce (serialize) and consume
(deserialize) Cursor on Target (CoT) messages.

### Initial goals:
- Support for base CoT schema
- Basic Rust structs for CoT messages, with serde support.

### TODOs
- [ ] better types for timestamps/dates. Currently just strings.
- [ ] support for detail contents (sub-schemas)

## References
1. Michael J. Kristan et al. November 2009 [Cursor on Target (CoT) Message Router User's Guide](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf).
2. Mitre Corporation, via CoTreceiver. Captured Nov 2024. [CoT Schema Definitions (XSD)](https://github.com/mdudel/CoTreceiver/tree/master/lib/xsd)
