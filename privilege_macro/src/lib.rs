extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
//use quote::quote;
use std::fs;
use std::io::prelude::*;
use syn::{parse_macro_input, AttributeArgs, File, ItemFn};

#[proc_macro_attribute]
pub fn require(attr: TokenStream, input: TokenStream) -> TokenStream {
    let origin = input.clone();
    let input_file = parse_macro_input!(input as ItemFn);
    let input_out = format!("{:#?}", input_file);

    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let attr_out = format!("{:#?}", attr_args);

    println!("{:#?}", input_file.decl.inputs[0]);
    fs::write("input.txt", &input_out);
    fs::write("attr.txt", &attr_out);
    origin
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
