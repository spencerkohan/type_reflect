# Serde Enum Tuples

*As a developer, I want my exported tuples to match those serialized by Serde, so my typescript types are fully interoperable with Rust*

## Acceptance Criteria

- [x] Tuple-type enums' associated data should generate ts as a value when there is only one item in the tuple
- [x] Tuple-type enums' associated data should generate ts as a tuple when there is more than one item in the tuple
