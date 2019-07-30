use snafu::{OptionExt, ResultExt, Snafu};
use std::{
    fmt::{self, Display},
    io::{Error as IoError, Write},
    process::{Command, Stdio},
};

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
                $crate::assert_tokens_eq(left, right, format_args!("`(left == right)`"))
            }
        }
    });
    ($left:expr , $right:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left = quote! { fn __wrapper() { #left_val } };
                let right = quote! { fn __wrapper() { #right_val } };
                $crate::assert_tokens_eq(left, right, format_args!("`(left == right)`: {}", $($arg)*))
            }
        }
    });
}

mod diff;

// I have no idea what dark magic this is, I only copied it from
// https://github.com/colin-kiegel/rust-pretty-assertions/blob/2f4058ac7fb5e24a923aef80851c6608b6683d0f/src/lib.rs#L84-L90
#[cfg(windows)]
#[ctor::ctor]
fn init() {
    let _ = output_vt100::try_init();
}

pub fn assert_tokens_eq(left_raw: impl Display, right_raw: impl Display, details: fmt::Arguments) {
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
             \n",
            details,
            diff::Comparison::new(left, right)
        );
    }
}

fn rustfmt(input: impl Display) -> Result<String, Error> {
    let mut fmt = Command::new("rustfmt")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context(Rustfmt { msg: "failed to spawn external process" })?;

    {
        let stdin = fmt.stdin.as_mut().context(RustfmtStdin { msg: "no stdin" })?;

        stdin
            .write_all(input.to_string().as_bytes())
            .context(Rustfmt { msg: "failed to write source to external process" })?;
    }

    let output =
        fmt.wait_with_output().context(Rustfmt { msg: "failed to read process output" })?;

    Ok(String::from_utf8(output.stdout).context(RustfmtReading)?)
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Error running rustfmt: {}: {}", msg, source))]
    Rustfmt { msg: String, source: IoError },
    #[snafu(display("Error running rustfmt: {}", msg))]
    RustfmtStdin { msg: String },
    #[snafu(display("Error reading rustfmt output: {}", source))]
    RustfmtReading { source: std::string::FromUtf8Error },
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

/// run this with `cargo test --lib -- test_le_diff --nocapture`
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
