use snafu::{OptionExt, ResultExt, Snafu};
use std::{
    io::{Error as IoError, Write},
    process::{Command, Stdio},
};

fn rustfmt(input: &str) -> Result<String, Error> {
    let mut fmt = Command::new("rustfmt")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context(Rustfmt { msg: "faild to spaw external process" })?;

    {
        let stdin = fmt.stdin.as_mut().context(RustfmtStdin { msg: "no stdin" })?;

        stdin
            .write_all(input.as_bytes())
            .context(Rustfmt { msg: "failed to write source to external process" })?;
    }

    let output =
        fmt.wait_with_output().context(Rustfmt { msg: "failed to read process output" })?;

    Ok(String::from_utf8(output.stdout).context(RustfmtReading)?)
}

pub fn assert_tokens_eq(left: &str, right: &str) -> Result<(), Error> {
    let left = rustfmt(left)?;
    let left = left.trim_start_matches("fn __wrapper() {\n");
    let left = left.trim_end_matches("\n}\n");

    let right = rustfmt(right)?;
    let right = right.trim_start_matches("fn __wrapper() {\n");
    let right = right.trim_end_matches("\n}\n");

    pretty_assertions::assert_eq!(left, right);
    Ok(())
}

#[macro_export]
macro_rules! assert_tokens_eq {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let left = quote! { fn __wrapper() { #left } };
        let right = $right;
        let right = quote! { fn __wrapper() { #right } };

        $crate::assert_tokens_eq(&left.to_string(), &right.to_string()).unwrap();
        // match  {
        //     // Err(e) => panic!("{}{}", e, e.source().map(Error::to_string).unwrap_or_default())
        // }
    }};
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error running rustfmt: {}", msg))]
    Rustfmt { msg: String, source: IoError },
    #[snafu(display("Error running rustfmt: {}", msg))]
    RustfmtStdin { msg: String },
    #[snafu(display("Error reading rustfmt output"))]
    RustfmtReading { source: std::string::FromUtf8Error },
}

#[test]
#[should_panic]
fn test_le_assert() {
    use quote::quote;
    assert_tokens_eq!(quote! { let x = 42; }, quote! { let x = 21; });
}
