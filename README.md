# [Tracing Asserts](https://crates.io/crates/tracing-asserts) (for Rust).

Small library providing assertion macros which emit [`tracing`](https://docs.rs/tracing) events at the error level.

Forked from the [`more_asserts`](https://docs.rs/more_asserts) crate.

## Usage

```rust
use tracing_asserts as ta;

#[derive(Debug, PartialEq, PartialOrd)]
enum Example { Foo, Bar }

ta::assert_le!(3, 4);
ta::assert_eq!(3, 2 + 1);
ta::assert_ne!(3, 2 - 1);
ta::assert_ge!(
    10, 10,
    "You can pass a message too (just like `assert_eq!`)",
);
ta::debug_assert_lt!(
    1.3, 4.5,
    "Format syntax is supported ({}).",
    "also like `assert_eq!`"
);

ta::assert_gt!(
    Example::Bar, Example::Foo,
    "It works on anything that implements PartialOrd and Debug!",
);
```

## License

[CC0 (public domain)](https://creativecommons.org/publicdomain/zero/1.0/).
