use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Expr, Ident, Token, braced, parse::Parse, punctuated::Punctuated};

use crate::init;

struct Field {
    name: Ident,
    #[allow(unused)]
    eq: Token![=],
    value: Expr,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct EmitArgs {
    handle_name: Ident,

    trait_name: Ident,

    #[allow(unused)]
    _as: syn::token::As,

    plugin_name: Ident,

    #[allow(unused)]
    brace: syn::token::Brace,

    fields: Punctuated<Field, Token![,]>,
}

impl Parse for EmitArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let braced;

        Ok(Self {
            handle_name: input.parse()?,
            trait_name: input.parse()?,
            _as: input.parse()?,
            plugin_name: input.parse()?,
            brace: braced!(braced in input),
            fields: braced.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

impl ToTokens for EmitArgs {
    fn to_tokens(
        &self,
        tokens: &mut proc_macro2::TokenStream,
    ) {
        let plugin = &self.plugin_name;
        let trt = &self.trait_name;
        let handle_name = &self.handle_name;

        let fields_as_expr_assign: TokenStream = self
            .fields
            .iter()
            .map(|field| {
                let name = &field.name;
                let expr = &field.value;
                quote::quote! {
                    #name: #expr,
                }
            })
            .collect();

        let struct_init = init(plugin);

        tokens.extend(quote::quote! {
            struct #handle_name;

            fn get() -> Box<dyn #trt> {
                Box::new(#handle_name)
            }

            inventory::submit!{
                #struct_init{
                    #fields_as_expr_assign
                    __get: get
                }
            }
        });
    }
}
