# Soma

Simple macro to simplify `Option` handling in Rust.

This is my answer to [RFC 1303](rust-lang/rfcs#1303) issue.

## Installation

Like other Crates you should use `cargo-edit` and then:

    cargo add soma

## Usage

This is equivalent to `try!` macro from `libstd`:

```rust
#[macro_use]
extern crate soma;

// …

let foo: Option<usize> = // …

let bar: usize = try_some!(foo); // This will return with `None` if `foo` is
                                 // `None`
// equivalent to:
//
// let bar: usize = match foo {
//     Some(val) => val,
//     None => return None,
// }

let baz: usize - try_some!(foo => return); // This will break execution if `foo`
                                           // is `None`
// equivalent to
//
// let baz: usize = match foo {
//     Some(val) => val,
//     None => return,
// }
```

## License

Check out [LICENSE][] file.
