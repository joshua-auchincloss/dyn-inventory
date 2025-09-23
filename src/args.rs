use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
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
    syn::custom_keyword!(macro_name);
    syn::custom_keyword!(handle_name);
}

#[allow(non_camel_case_types)]
enum ExtraOpts {
    macro_name { value: Ident },
    handle_name { value: Ident },
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
            input & [macro_name, handle_name]
        }
    }
}

pub struct Args {
    pub trait_name: Ident,
    pub struct_name: Ident,
    pub generic_param: Ident,

    #[allow(unused)]
    brace: syn::token::Brace,

    pub fields: Punctuated<Field, Token![,]>,

    #[allow(unused)]
    term: Token![;],

    opts: Punctuated<ExtraOpts, Token![,]>,

    pub macro_name: Ident,
    pub handle_name: Ident,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let trait_name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;

        let struct_name: Ident = input.parse()?;
        let _: Token![<] = input.parse()?;
        let generic_param: Ident = input.parse()?;
        let _: Token![>] = input.parse()?;

        let macro_name = Ident::new(
            &format!("{}", struct_name).to_case(Case::Snake),
            input.span(),
        );

        let content;
        let brace = braced!(content in input);
        let fields = content.parse_terminated(Field::parse, Token![,])?;

        let term = input.parse()?;

        let opts = if !input.is_empty() {
            input.parse_terminated(ExtraOpts::parse, Token![,])?
        } else {
            Punctuated::new()
        };

        let handle_name = Ident::new(&format!("Handle{}", trait_name), input.span());

        let mut this = Self {
            macro_name,
            trait_name,
            struct_name,
            generic_param,
            brace,
            fields,
            term,
            opts,
            handle_name,
        };

        for opt in &this.opts {
            match opt {
                ExtraOpts::macro_name { value, .. } => {
                    this.macro_name = value.clone();
                },
                ExtraOpts::handle_name { value, .. } => this.handle_name = value.clone(),
            }
        }

        Ok(this)
    }
}

impl Args {
    pub fn collect_fields<
        S: Into<String> + Clone,
        F: Copy + Fn(&Visibility, &Ident, &RefOrTy) -> TokenStream,
    >(
        &self,
        f: F,
        generic: &S,
    ) -> TokenStream {
        self.collect_fields_or_generic(f, f, generic.clone().into())
    }

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
