Enumoid
=======

This crate is a Rust library which provides containers indexed by enums.

## Dependency

```toml
[dependencies]
enumoid = "0.3"
```

# Using Enumoid

The Enumoid trait provides a mapping between the values which inhabit a type
and the integers between 0 and n, where n is the number of distinct values.
This is used to provide several container data structures backed by fixed-size
arrays and generic over an Enumoid index of type `T`.

For example, a total mapping:

```rust
let mut map = EnumMap::<FooBar, String>::new();
map[FooBar::Foo] = "Hello".to_string();
```

The Enumoid trait can be implemented for any enum type using the Enumoid derive
macro.

```rust
#[derive(Enumoid)]
enum FooBar { Foo, Bar }
```

## Licence

This crate is licensed under the Apache License, Version 2.0 (see
LICENCE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>) or the MIT
licence (see LICENCE-MIT or <http://opensource.org/licenses/MIT>), at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
