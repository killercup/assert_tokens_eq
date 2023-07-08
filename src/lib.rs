#![doc = include_str!("../README.md")]

mod assert_tokens_eq;
mod assert_tokens_eq_v;
mod formatting;
mod rustfmt;

// This code is needed to enable ANSI escape codes in Windows consoles.
// Source: https://github.com/colin-kiegel/rust-pretty-assertions/blob/2f4058ac7fb5e24a923aef80851c6608b6683d0f/src/lib.rs#L84-L90
#[cfg(windows)]
#[ctor::ctor]
fn init() {
    let _ = output_vt100::try_init();
}

pub use crate::assert_tokens_eq::assert_tokens_eq;
pub use crate::assert_tokens_eq_v::assert_tokens_eq_v;
