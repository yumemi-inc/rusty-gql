mod resolver;
mod scalar;
mod union;
mod utils;

use proc_macro::{self, TokenStream};
use scalar::generate_scalar;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemImpl};
use union::generate_union;

use crate::resolver::generate_resolver;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Resolver(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(input as ItemImpl);
    let args = parse_macro_input!(args as AttributeArgs);

    let expanded = match generate_resolver(&mut item_impl, &args[..]) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(Scalar)]
#[allow(non_snake_case)]
pub fn scalar_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    let expanded = match generate_scalar(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}

#[proc_macro_derive(Union)]
#[allow(non_snake_case)]
pub fn union_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    let expanded = match generate_union(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    };
    expanded
}
