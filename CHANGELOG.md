# Changelog

## Enumoid 0.5.0 (2026-06-29)

### Added
- Added generate_enumoid! macro.
- Added EnumMap::set and set_by_index.
- Added insert, remove, swap, and iterator methods to EnumOptionMap.
- Added Debug and Serialize/Deserialize impls for EnumOptionMap.
- Added EnumSet::new_all.
- Added missing by_index versions of operations.
- Added Extend and From/IntoIterator trait impls.

### Changed
- Set minimum supported Rust version to 1.85 and 2024 edition.
- Changed EnumOptionMap::set to return the old value.
- Changed EnumSet to iterate over set members rather than a map of bools.
- Changed EnumSize to take self by value.
- Replaced EnumVec::push with try_push.
- Removed invalid ExactSizeIterator impls from EnumSet and EnumOptionMap.

### Fixed
- Fixed division by zero in EnumSet::all when there are no partial words.
- Fixed EnumSet iterator skipping a word when the first is empty.
- Fixed off-by-one error in EnumSize::iter_from_until.
- Fixed EnumSize::iter_until ignoring the size.

## Enumoid 0.4.0 (2024-02-07)

### Added
- Added support for unary structs and enum variants.
- Added generic bitset word size to EnumSet and EnumOptionMap.
- Added EnumIndex type and related container methods.

### Changed
- Renamed EnumFlags to EnumSet.
- Renamed Size to EnumSize.
- Removed support for uninhabited enums.

## Enumoid 0.3.0 (2023-08-31)

### Added
- Added swap_remove function to EnumVec.
- Added clear function to EnumOptionMap.

### Changed
- Changed Option<Size> to Size in EnumVec::new_with.
- Changed proc macro to use syn 2.0.
- Removed dependency on num-traits.
- Removed default features from serde dependency.

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
