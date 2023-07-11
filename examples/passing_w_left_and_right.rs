use assert_tokens_eq::assert_tokens_eq;
use quote::quote;

fn main() {
    let ts1 = quote! {
        let s = "hello";
    };
    let ts2 = quote! {
        let s = "hello";
    };
    assert_tokens_eq!(ts1, ts2);
}
