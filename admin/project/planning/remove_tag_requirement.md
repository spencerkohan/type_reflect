# Remove Tag Requirement

Currently enums require a serde `tag` attribute when there is associated data.

This requirement should be relaxed:
- support the untagged union typescript case
- the untagged serde representation is the default enum representation for Swift
