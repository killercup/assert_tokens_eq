use assert_tokens_eq::{assert_tokens_eq_v, Opts};
use quote::quote;

fn main() {
    let ts1 = quote! {
        let s = "hewwo";
    };
    let ts2 = quote! {
        let s = "hello";
    };

    let mut opts = Opts::default();
    opts.wrap_in_fn = false;
    opts.apply_rustfmt = false;

    assert_tokens_eq_v!(ts1, ts2, opts: opts);
}
