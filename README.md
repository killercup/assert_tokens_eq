# assert_tokens_eq

[![Latest Version](https://img.shields.io/crates/v/assert_tokens_eq.svg)][`assert_tokens_eq`]
[![Downloads](https://img.shields.io/crates/d/assert_tokens_eq.svg)][`assert_tokens_eq`]
[![Documentation](https://docs.rs/assert_tokens_eq/badge.svg)][`assert_tokens_eq`/docs]
[![License](https://img.shields.io/crates/l/assert_tokens_eq.svg)][`assert_tokens_eq`/license]
[![Dependency Status](https://deps.rs/repo/github/JohnScience/assert_tokens_eq/status.svg)][`assert_tokens_eq`/dep_status]

Like Rust's built-in [`assert_eq`] macro, but for token streams.
Passes them through [`rustfmt`], and shows a pretty diff (powered by [`pretty_assertions`]).

## How it works

You write:

```rust, ignore
let got = something_that_generates_rust_code();
let expected = quote! {
    #[no_mangle]
    extern "C" fn foo(
        input: *const ::libc::c_char,
        input_len: ::libc::size_t,
        input3: *const ::libc::c_int,
        input3_len: ::libc::size_t,
    ) {
        fn foo(input: Arc<str>, input2: Arc<str>) {}
        unimplemented!()
    }
};
assert_tokens_eq!(got, expected);
```

and you get:

![Screenshot of `assert_token_eq!` output](./screenshot.png)

Note that this crate also provides a [`assert_tokens_eq_v`] macro that additionally prints the full token streams when the assertion fails.

![Screenshot of `assert_token_eq_v!` output](./screenshot2.png)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[`assert_eq`]: https://doc.rust-lang.org/std/macro.assert_eq.html
[`pretty_assertions`]: https://crates.io/crates/pretty_assertions
[`rustfmt`]: https://github.com/rust-lang/rustfmt#rustfmt----
[`assert_tokens_eq_v`]: https://docs.rs/assert_tokens_eq/latest/assert_tokens_eq/macro.assert_tokens_eq_v.html
[`assert_tokens_eq`]: https://crates.io/crates/assert_tokens_eq
[`assert_tokens_eq`/docs]: https://docs.rs/assert_tokens_eq
[`assert_tokens_eq`/license]: https://github.com/JohnScience/assert_tokens_eq#license
[`assert_tokens_eq`/dep_status]: https://deps.rs/repo/github/JohnScience/assert_tokens_eq
