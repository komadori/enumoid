Enumoid
=======

This crate is a Rust library which can establish a mapping between the values that inhabit a type and the integers between 0 and n, where n is the number of distinct values. This is used to provide a range of utility functions for traversing the space of values and several container data structures indexed by such values.

## Dependency

```toml
[dependencies]
enumoid = "0.4"
```

## Deriving Enumoid
In order to use a type with this crate, it must implement the Enumoid trate using the eponymous derive proc macro.

Enumoid can be derived for enum types with unit variants:

```rust
use enumoid::Enumoid;

#[derive(Enumoid)]
enum Weekday { Monday, Tuesday, Wednesday, Thursday, Friday }
```

Enumoid can also be derived for tuple variants with a single field whose type also implements Enumoid:

```rust
# use enumoid::Enumoid;
# #[derive(Enumoid)]
# enum Weekday { Monday, Tuesday, Wednesday, Thursday, Friday }
#[derive(Enumoid)]
enum Day { Work(Weekday), Rest(Weekend) }

#[derive(Enumoid)]
enum Weekend { Saturday, Sunday }
```

The field may not have a generic type as this would require currently unstable aspects of const generics.

Enumoids can also be derived for unit structs and for tuple structs with a single field whose type implements Enumoid:

```rust
# use enumoid::Enumoid;
# #[derive(Enumoid)]
# enum Day { Placeholder }
#[derive(Enumoid)]
struct AnyDay;

#[derive(Enumoid)]
struct EveryDay(Day);
```

By default, a u8 is used to represent the number of values inhabiting an Enumoid. If you want to derive Enumoid for a type with more than 255 values, you can specify a wider type with the `index_type` helper attribute.

```
# use enumoid::Enumoid;
#[derive(Enumoid)]
#[index_type(u32)]
enum Massive { A, /*...*/ }
```

## Traversing Enumoids

The Enumoid trait provides a range of utility functions for traversing through value space. They allow you to find the next or previous value, with or without wrapping. For example:

```rust
# use enumoid::Enumoid;
# #[derive(Enumoid, Debug, PartialEq)]
# enum Day { Work(Weekday), Rest(Weekend) }
# #[derive(Enumoid, Debug, PartialEq)]
# enum Weekday { Monday, Tuesday, Wednesday, Thursday, Friday }
# #[derive(Enumoid, Debug, PartialEq)]
# enum Weekend { Saturday, Sunday }
let tomorrow = Day::Work(Weekday::Friday).next();
assert_eq!(tomorrow, Some(Day::Rest(Weekend::Saturday)));

let yesterday = Day::Work(Weekday::Monday).prev_wrapped();
assert_eq!(yesterday, Day::Rest(Weekend::Sunday));
```

Another useful operation is to iterate through all the values:

```rust
# use enumoid::Enumoid;
# #[derive(Enumoid, Debug, PartialEq)]
# enum Weekend { Saturday, Sunday }
assert_eq!(
    Weekend::iter().collect::<Vec<Weekend>>(),
    vec![Weekend::Saturday, Weekend::Sunday])
```

## Enumoid-indexed Containers

This crate provides a range of container types which use an Enumoid as a key. They are all backed internally by fixed-size arrays and so do not allocate. For example, `EnumMap` provides a total mapping from the values of an Enumoid to some other type `T`:

```rust
# use enumoid::Enumoid;
# use enumoid::EnumMap;
# #[derive(Enumoid)]
# enum FooBar { Foo, Bar }
let mut map = EnumMap::<FooBar, String>::new();
map[FooBar::Foo] = "Hello".to_string();
```

`EnumOptionMap` is similar, but it provides a partial mapping wherein elements may be absent. `EnumVec` is also a partial mapping, but operates like a vector wherein any present elements are contiguous with the first value of the Enumoid. `EnumFlags` is specialised for storing booleans.

## Licence

This crate is licensed under the Apache License, Version 2.0 (see
LICENCE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>) or the MIT
licence (see LICENCE-MIT or <http://opensource.org/licenses/MIT>), at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
