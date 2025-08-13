use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(EnumFlags)]
pub fn enum_flags_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        Data::Enum(e) => e.variants,
        _ => panic!("EnumFlags can only be derived for enums"),
    };

    let mut consts = vec![];

    for variant in variants {
        let ident = variant.ident;
        let value = match variant.discriminant {
            Some((_, ref expr)) => expr,
            None => panic!("Enum variants must have explicit values"),
        };
        consts.push(quote! {
            pub const #ident: usize = #value;
        });
    }

    let expanded = quote! {
        impl #name {
            #(#consts)*
        }
    };

    TokenStream::from(expanded)
}
