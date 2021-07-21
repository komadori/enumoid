# Changelog

## Enumoid 0.2.0 (2021-07-21)

### Added
- Added Size struct for dealing with subsets of enum values.
- Added family of next and prev functions.
- Added double-ended iterator trait for EnumMap and EnumVec.
- Added exact iterator trait for all iterators.

### Changed
- Improved performance.
- Removed wrapped_add and checked_add functions.
- Merged the EnumFlagsHelper trait into Enumoid.
- Changed and hidden many members on the Enumoid and EnumArrayHelper traits.

### Fixed
- Fixed EnumFlags with more than 256 flags.

## Enumoid 0.1.1 (2021-05-04)

### Added
- Added Enumoid1 trait to provide FIRST and LAST consts.

## Enumoid 0.1.0 (2021-04-02)
- Initial release.
