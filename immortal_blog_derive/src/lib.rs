extern crate proc_macro;
extern crate quote;
extern crate serde_json;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse_macro_input, AttributeArgs, ExprLit, ItemFn, Lit, Meta, MetaNameValue, NestedMeta,
};

#[proc_macro_attribute]
pub fn require_permissions(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let mut permissions = HashMap::new();
    attr_args.iter().for_each(|arg| match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            lit: Lit::Str(ref value),
            ident: ref name,
            eq_token: _,
        })) => {
            let key = name.to_string();
            if let Lit::Int(ref level) = value.parse::<ExprLit>().unwrap().lit {
                permissions.insert(key, level.value());
            } else {
                panic!("Unexpected config with require_permissions");
            }
        }
        _ => panic!("Unexpected config with require_permissions"),
    });

    let fn_name = input_fn.ident;
    let fn_block = input_fn.block.stmts.iter();
    let fn_decl = input_fn.decl;
    let fn_inputs = fn_decl.inputs;
    let fn_output = fn_decl.output;

    let permissions = serde_json::to_string(&permissions).unwrap();
    let output = quote! {
        pub fn #fn_name(req: HttpRequest,#fn_inputs) #fn_output {
            let require_permissions = serde_json::from_str(#permissions).unwrap();
            utils::check_permissions(require_permissions,&req).and_then(move|_|{
            #(
                #fn_block
            )*
            })
        }
    };
    output.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
