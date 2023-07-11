use core::fmt::Write;

mod formatting;
mod rustfmt;

// This code is needed to enable ANSI escape codes in Windows consoles.
// Source: https://github.com/colin-kiegel/rust-pretty-assertions/blob/2f4058ac7fb5e24a923aef80851c6608b6683d0f/src/lib.rs#L84-L90
#[cfg(windows)]
#[ctor::ctor]
fn init() {
    let _ = output_vt100::try_init();
}

use self::rustfmt::rustfmt;
use std::fmt::{self, Display};

#[macro_export]
macro_rules! assert_tokens_eq {
    ($left:expr , $right:expr,) => ({
        assert_tokens_eq!($left, $right)
    });
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = quote! { fn __wrapper() { #left_val } };
                let right = quote! { fn __wrapper() { #right_val } };
                let opts = $crate::Opts::default();
                $crate::assert_tokens_eq(left, right, opts, format_args!("`(left == right)`"))
            }
        }
    });
    ($left:expr , $right:expr, opts: $opts:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #left_val } }
                } else {
                    quote! { #left_val }
                };
                let right = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #right_val } }
                } else {
                    quote! { #right_val }
                };
                $crate::assert_tokens_eq(left, right, $opts, format_args!("`(left == right)`"))
            }
        }
    });
    ($left:expr , $right:expr, opts: $opts:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #left_val } }
                } else {
                    quote! { #left_val }
                };
                let right = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #right_val } }
                } else {
                    quote! { #right_val }
                };
                $crate::assert_tokens_eq(left, right, $opts, format_args!("`(left == right)`: {}", $($arg)*))
            }
        }
    });
    ($left:expr , $right:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = quote! { fn __wrapper() { #left_val } };
                let right = quote! { fn __wrapper() { #right_val } };
                let opts = Opts::default();
                $crate::assert_tokens_eq(left, right, opts, format_args!("`(left == right)`: {}", $($arg)*))
            }
        }
    });
}

#[macro_export]
macro_rules! assert_tokens_eq_v {
    ($left:expr , $right:expr,) => ({
        assert_tokens_eq_v!($left, $right)
    });
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = quote! { fn __wrapper() { #left_val } };
                let right = quote! { fn __wrapper() { #right_val } };
                let mut opts = $crate::Opts::default();
                opts.show_full_left = true;
                opts.show_full_right = true;
                $crate::assert_tokens_eq(left, right, opts, format_args!("`(left == right)`"))
            }
        }
    });
    ($left:expr , $right:expr, opts: $opts:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #left_val } }
                } else {
                    quote! { #left_val }
                };
                let right = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #right_val } }
                } else {
                    quote! { #right_val }
                };
                let mut opts = $opts;
                opts.show_full_left = true;
                opts.show_full_right = true;
                $crate::assert_tokens_eq(left, right, opts, format_args!("`(left == right)`"))
            }
        }
    });
    ($left:expr , $right:expr, opts: $opts:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #left_val } }
                } else {
                    quote! { #left_val }
                };
                let right = if $opts.wrap_in_fn {
                    quote! { fn __wrapper() { #right_val } }
                } else {
                    quote! { #right_val }
                };
                let mut opts = $opts;
                opts.show_full_left = true;
                opts.show_full_right = true;
                $crate::assert_tokens_eq(left, right, opts, format_args!("`(left == right)`: {}", $($arg)*))
            }
        }
    });
    ($left:expr , $right:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = quote! { fn __wrapper() { #left_val } };
                let right = quote! { fn __wrapper() { #right_val } };
                let mut opts = $crate::Opts::default();
                opts.show_full_left = true;
                opts.show_full_right = true;
                $crate::assert_tokens_eq(left, right, opts, format_args!("`(left == right)`: {}", $($arg)*))
            }
        }
    });
}

/// Options that can be supplied to `assert_tokens_eq` macro.
#[non_exhaustive]
pub struct Opts {
    /// Whether to wrap the input in a function before the formatting and comparison.
    ///
    /// This is a legacy option that exists to support the old implementation of [`assert_tokens_eq`]
    /// macro.
    pub wrap_in_fn: bool,
    /// Whether to apply rustfmt to the input before the comparison.
    ///
    /// While it can make equivalent but not identical code pass the test,
    /// it can't deal with code that is not syntactically valid on its own, e.g.
    /// `0usize, 0usize, 0usize, 255usize,` or even `let s = "A string"` because
    /// the latter would be the case of "global let".
    pub apply_rustfmt: bool,
    /// Whether to show the full left side of the comparison.
    pub show_full_left: bool,
    /// Whether to show the full right side of the comparison.
    pub show_full_right: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            wrap_in_fn: true,
            apply_rustfmt: true,
            show_full_left: false,
            show_full_right: false,
        }
    }
}

pub fn assert_tokens_eq(
    left_raw: impl Display,
    right_raw: impl Display,
    opts: Opts,
    details: fmt::Arguments,
) {
    let left = if opts.apply_rustfmt { rustfmt(left_raw).unwrap() } else { left_raw.to_string() };

    let left = if opts.wrap_in_fn {
        left.trim_start_matches("fn __wrapper() {\n").trim_end_matches("\n}\n")
    } else {
        left.as_str()
    };

    let right =
        if opts.apply_rustfmt { rustfmt(right_raw).unwrap() } else { right_raw.to_string() };

    let right = if opts.wrap_in_fn {
        right.trim_start_matches("fn __wrapper() {\n").trim_end_matches("\n}\n")
    } else {
        right.as_str()
    };

    if *left != *right {
        let mut s = String::new();
        write!(s, "assertion failed: {}\n\n", details).unwrap();
        write!(s, "{}", formatting::Comparison::new(left, right)).unwrap();

        let left = if opts.show_full_left { Some(left) } else { None };
        let right = if opts.show_full_right { Some(right) } else { None };
        if let Some(full) = formatting::FullTokenStrs::new(left, right) {
            write!(s, "{full}").unwrap();
        }

        panic!("{}", s);
    }
}

#[test]
fn test_cursed_code() {
    use quote::quote;
    let got = quote! {
        # [ no_mangle ] extern "C" fn foo ( input : * const :: libc :: c_char , input_len : :: libc :: size_t , input2 : * const :: libc :: c_char , input2_len : :: libc :: size_t ) { fn foo ( input : Arc < str > , input2 : Arc < str > ) { } unimplemented ! ( ) }
    };
    let expected = quote! {
        #[no_mangle]
        extern "C" fn foo(
            input: *const ::libc::c_char,
            input_len: ::libc::size_t,
            input2: *const ::libc::c_char,
            input2_len: ::libc::size_t,
        ) {
            fn foo(input: Arc<str>, input2: Arc<str>) {}
            unimplemented!()
        }

    };
    assert_tokens_eq!(got, expected);
}

/// run this with
/// `cargo test --lib -- test_le_diff --nocapture`
#[test]
#[should_panic]
fn test_le_diff() {
    use quote::quote;
    let got = quote! {
        # [ no_mangle ] extern "C" fn foo ( input : * const :: libc :: c_char , input_len : :: libc :: size_t , input2 : * const :: libc :: c_char , input2_len : :: libc :: size_t ) { fn foo ( input : Arc < str > , input2 : Arc < str > ) { } unimplemented ! ( ) }
    };
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
}
