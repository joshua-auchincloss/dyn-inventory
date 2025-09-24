use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    Ident, Token, TypePath, TypeReference, Visibility, braced, parse::Parse, punctuated::Punctuated,
};

pub enum RefOrTy {
    Ty(TypePath),
    Ref(TypeReference),
}

impl ToTokens for RefOrTy {
    fn to_tokens(
        &self,
        tokens: &mut proc_macro2::TokenStream,
    ) {
        tokens.extend(match self {
            Self::Ref(reffed) => {
                let lt = reffed
                    .lifetime
                    .clone()
                    .map(|ok| quote::quote! {#ok})
                    .unwrap_or_default();

                let ty = *reffed.elem.clone();

                let mutability = reffed
                    .mutability
                    .map(|mt| quote::quote!(#mt))
                    .unwrap_or_default();

                quote::quote! {
                    & #lt #mutability #ty
                }
            },
            Self::Ty(ty) => quote::quote!(#ty),
        });
    }
}

impl Parse for RefOrTy {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![&]) {
            Ok(Self::Ref(input.parse()?))
        } else {
            Ok(Self::Ty(input.parse()?))
        }
    }
}

pub struct Field {
    pub vis: Visibility,
    pub name: Ident,
    #[allow(unused)]
    sep: Token![:],
    pub ty: RefOrTy,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            vis: input
                .parse()
                .unwrap_or(Visibility::Inherited),
            name: input.parse()?,
            sep: input.parse()?,
            ty: input.parse()?,
        })
    }
}

mod kw {
    syn::custom_keyword!(init_name);
}

#[allow(non_camel_case_types)]
enum ExtraOpts {
    init_name { value: Ident },
}

macro_rules! kws {
    (
        $input: ident & $kw: ident
    ) => {

        if $input.peek(kw::$kw) {
            let _: kw::$kw = $input.parse()?;
            let _: Token![=] = $input.parse()?;
            return Ok(Self::$kw {
                value: $input.parse()?,
            });
        }
    };
    ( $input: ident & [$(
        $kw: ident
    ), + $(,)?]) => {
        $(
            kws!{ $input & $kw }
        )*
        Err(
            syn::Error::new(
                $input.span(),
                "unknown keyword",
            )
        )
    };
}

impl Parse for ExtraOpts {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        kws! {
            input & [init_name]
        }
    }
}

pub struct Args {
    pub struct_name: Ident,

    pub trait_name: Ident,

    pub generic_param: Ident,

    #[allow(unused)]
    brace: syn::token::Brace,

    pub fields: Punctuated<Field, Token![,]>,

    #[allow(unused)]
    term: Option<Token![;]>,

    opts: Punctuated<ExtraOpts, Token![,]>,

    pub init_name: Ident,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_name: Ident = input.parse()?;
        let _: Token![<] = input.parse()?;

        let generic_param: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;

        let trait_name: Ident = input.parse()?;
        let _: Token![>] = input.parse()?;

        let init_name = crate::init(&struct_name);

        let content;
        let brace = braced!(content in input);
        let fields = content.parse_terminated(Field::parse, Token![,])?;

        let term = match input.parse() {
            Ok(cont) => Some(cont),
            _ => {
                if !input.is_empty() {
                    return Err(syn::Error::new(input.span(), "expected `;`"));
                } else {
                    None
                }
            },
        };

        let opts = if !input.is_empty() {
            input.parse_terminated(ExtraOpts::parse, Token![,])?
        } else {
            Punctuated::new()
        };

        let mut this = Self {
            init_name,
            trait_name,
            struct_name,
            generic_param,
            brace,
            fields,
            term,
            opts,
        };

        for opt in &this.opts {
            match opt {
                ExtraOpts::init_name { value, .. } => {
                    this.init_name = value.clone();
                },
            }
        }

        Ok(this)
    }
}

impl Args {
    pub fn collect_fields_or_generic<
        F: Fn(&Visibility, &Ident, &RefOrTy) -> TokenStream,
        G: Fn(&Visibility, &Ident, &RefOrTy) -> TokenStream,
    >(
        &self,
        fh: F,
        gh: G,
        generic: String,
    ) -> TokenStream {
        let mut tt = quote::quote! {};
        let mut gg = quote::quote! {};
        for f in &self.fields {
            let field_ty = &f.ty;
            if quote::quote!(#field_ty).to_string() == generic {
                gg.extend(gh(&f.vis, &f.name, field_ty));
            } else {
                tt.extend(fh(&f.vis, &f.name, field_ty));
            }
        }
        tt.extend(gg);
        tt
    }
}

impl ToTokens for Args {
    fn to_tokens(
        &self,
        tokens: &mut TokenStream,
    ) {
        let trt = &self.trait_name;
        let strct = &self.struct_name;
        let generic = &self.generic_param;
        let generic_str = generic.to_string();
        let struct_init = &self.init_name;

        let fields_init: proc_macro2::TokenStream = self.collect_fields_or_generic(
            |vis, name, ty| {
                quote::quote! {
                    #vis #name: #ty,
                }
            },
            |vis, _, _| {
                quote::quote! {
                    #vis __get: fn() -> Box<dyn #trt>,
                }
            },
            generic_str.clone(),
        );

        let fields_final: proc_macro2::TokenStream = self.collect_fields_or_generic(
            |vis, name, ty| {
                quote::quote! {
                    #vis #name: #ty,
                }
            },
            |vis, name, _| {
                quote::quote! {
                    #vis #name: Box<dyn #trt>,
                }
            },
            generic_str.clone(),
        );

        let fields_as_into = self.collect_fields_or_generic(
            |_, name, _| {
                quote::quote! {
                    #name: value.#name,
                }
            },
            |_, name, _| {
                quote::quote! {
                    #name: (value.__get)(),
                }
            },
            generic_str.clone(),
        );

        let struct_def = quote::quote! {
            #[derive(Clone)]
            pub struct #struct_init {
                #fields_init
            }

            impl From<#struct_init> for #strct {
                fn from(value: #struct_init) -> Self {
                    Self {
                        #fields_as_into
                    }
                }
            }

            pub struct #strct {
                #fields_final
            }
        };

        let collect = quote::quote! {
            inventory::collect!{
                #struct_init
            }
        };

        let plugin_collector = Ident::new(&format!("{}Collector", strct), Span::call_site());

        let plugin_collector = quote::quote! {
            pub struct #plugin_collector{
                pub plugins: Vec<#strct>
            }

            impl #plugin_collector {
                pub fn new() -> Self {
                    Self::new_with(|_| {})
                }

                pub fn new_with<F: Fn(&mut #strct)>(with: F) -> Self {
                    let mut plugins = vec![];
                    for plugin in inventory::iter::<#struct_init> {
                        let mut plugin: #strct = plugin.clone().into();
                        with(&mut plugin);
                        plugins.push(plugin);
                    }
                    Self { plugins }
                }
            }
        };

        tokens.extend(quote::quote! {
            #struct_def

            #collect

            #plugin_collector
        });
    }
}
