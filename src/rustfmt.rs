use snafu::{OptionExt, ResultExt, Snafu};
use std::{
    fmt::Display,
    io::{Error as IoError, Write},
    process::{Command, Stdio},
};

pub(crate) fn rustfmt(input: impl Display) -> Result<String, Error> {
    let mut fmt = Command::new("rustfmt")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context(RustfmtSnafu { msg: "failed to spawn external process" })?;

    {
        let stdin = fmt.stdin.as_mut().context(RustfmtStdinSnafu { msg: "no stdin" })?;

        stdin
            .write_all(input.to_string().as_bytes())
            .context(RustfmtSnafu { msg: "failed to write source to external process" })?;
    }

    let output =
        fmt.wait_with_output().context(RustfmtSnafu { msg: "failed to read process output" })?;

    String::from_utf8(output.stdout).context(RustfmtReadingSnafu)
}

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("Error running rustfmt: {msg}: {source}"))]
    Rustfmt { msg: String, source: IoError },
    #[snafu(display("Error running rustfmt: {msg}"))]
    RustfmtStdin { msg: String },
    #[snafu(display("Error reading rustfmt output: {source}"))]
    RustfmtReading { source: std::string::FromUtf8Error },
}
