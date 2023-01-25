# Changelog

## Enumoid 0.2.3 (2023-01-25)

### Added
- Added PartialEq, Eq, and Hash impls to all containers.

### Fixed
- Fixed missing Drop impl for EnumOptionMap.

## Enumoid 0.2.2 (2022-08-31)

### Added
- Added get_mut function to EnumMap, EnumOptionMap, and EnumVec.

### Fixed
- Fixed unwanted loop vectorisation over small backing arrays.

## Enumoid 0.2.1 (2021-08-02)

### Added
- Added pop function to EnumVec.

### Changed
- Fixed bad debug assertion in EnumVec's as_slice functions.

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
- Changed and hid many members on the Enumoid and EnumArrayHelper traits.

### Fixed
- Fixed EnumFlags with more than 256 flags.

## Enumoid 0.1.1 (2021-05-04)

### Added
- Added Enumoid1 trait to provide FIRST and LAST consts.

## Enumoid 0.1.0 (2021-04-02)
- Initial release.
