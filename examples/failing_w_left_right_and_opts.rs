use assert_tokens_eq::{assert_tokens_eq, Opts};
use quote::quote;

fn main() {
    let ts1 = quote! {
        let s = "hewwo";
    };
    let ts2 = quote! {
        let s = "hello";
    };

    // This is pointless but we can do it anyway
    #[allow(unused_mut)]
    let mut opts = Opts::default();
    // opts.wrap_in_fn = false;
    // opts.show_full_left = true;
    // opts.show_full_right = true;

    // Supplying the options like this is a bit verbose but
    // it was the only *easy* way to preserve the original API.
    assert_tokens_eq!(ts1, ts2, opts: opts);
}