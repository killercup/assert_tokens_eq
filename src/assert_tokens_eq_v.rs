use crate::formatting;
use crate::rustfmt::rustfmt;
use std::fmt::{self, Display};

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
                $crate::assert_tokens_eq_v(left, right, format_args!("`(left == right)`"))
            }
        }
    });
    ($left:expr , $right:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = quote! { fn __wrapper() { #left_val } };
                let right = quote! { fn __wrapper() { #right_val } };
                $crate::assert_tokens_eq_v(left, right, format_args!("`(left == right)`: {}", $($arg)*))
            }
        }
    });
}

pub fn assert_tokens_eq_v(
    left_raw: impl Display,
    right_raw: impl Display,
    details: fmt::Arguments,
) {
    let left = rustfmt(left_raw).unwrap();
    let left = left.trim_start_matches("fn __wrapper() {\n");
    let left = left.trim_end_matches("\n}\n");

    let right = rustfmt(right_raw).unwrap();
    let right = right.trim_start_matches("fn __wrapper() {\n");
    let right = right.trim_end_matches("\n}\n");

    if *left != *right {
        panic!(
            "assertion failed: {}\
             \n\
             \n{}\
             \n{}\
             \n",
            details,
            formatting::Comparison::new(left, right),
            formatting::FullTokenStrs::new(left, right),
        );
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
    assert_tokens_eq_v!(got, expected);
}

/// run this with `cargo test --lib -- assert_tokens_eq_v::test_le_diff --nocapture`
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
    assert_tokens_eq_v!(got, expected);
}
