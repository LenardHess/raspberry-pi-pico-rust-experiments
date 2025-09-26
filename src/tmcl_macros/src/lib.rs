//! Macros for the TMCL crate

use proc_macro::TokenStream;
use quote;
use syn;

use darling::export::NestedMeta;
use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct AxisParameterArgs {
    index: u16,
}

#[proc_macro_attribute]
pub fn axis_parameter(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match AxisParameterArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let ap_index = args.index;

    let item = syn::parse_macro_input!(input as syn::ItemFn);

    // Verify the macro arguments

    let attrs = &item.attrs;
    let vis = &item.vis;
    let sig = &item.sig;
    let block = &item.block;

    let varname = quote::format_ident!("AP_{}_{}_", &sig.ident, ap_index);

    quote::quote!(
        #[used]
        #[unsafe(link_section = ".ap_entries")]
        #[allow(non_upper_case_globals)]
        static #varname : u32 = 0;

        #(#attrs)*
        #vis #sig #block
    )
    .into()
}