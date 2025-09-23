mod args;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::Ident;

use crate::args::Args;

#[proc_macro]
pub fn dyn_inventory(tok: TokenStream) -> TokenStream {
    let args: Args = match syn::parse(tok) {
        Ok(parse) => parse,
        Err(e) => {
            return e.into_compile_error().into();
        },
    };

    let trt = &args.trait_name;
    let strct = &args.struct_name;
    let handle = &args.handle_name;
    let generic = &args.generic_param;
    let macro_name = &args.macro_name;
    let generic_str = generic.to_string();

    let fields: proc_macro2::TokenStream = args.collect_fields(
        |vis, name, ty| {
            quote::quote! {
                #vis #name: #ty,
            }
        },
        &generic_str,
    );

    let fields_as_args = args.collect_fields(
        |_, name, ty| {
            quote::quote! {
                #name: #ty,
            }
        },
        &generic_str,
    );

    let fields_as_assign = args.collect_fields(
        |_, name, _| {
            quote::quote! {
                #name: #name,
            }
        },
        &generic_str,
    );

    let fields_as_expr = args.collect_fields_or_generic(
        |_, name, _| {
            quote::quote! {
                #name = $#name: expr;
            }
        },
        |_, _, _| Default::default(),
        generic_str.clone(),
    );

    let fields_as_expr_call = args.collect_fields_or_generic(
        |_, name, _| {
            quote::quote! {
                #name = $#name;
            }
        },
        |_, _, _| Default::default(),
        generic_str.clone(),
    );

    let fields_as_expr_assign = args.collect_fields_or_generic(
        |_, name, _| {
            quote::quote! {
                $#name,
            }
        },
        |_, _, _| Default::default(),
        generic_str.clone(),
    );

    let field_as_reassign = args.collect_fields_or_generic(
        |_, name, _| {
            quote::quote! {
                plugin.#name,
            }
        },
        |_, name, _| {
            quote::quote! {
                (plugin.#name)()
            }
        },
        generic_str.clone(),
    );

    let struct_def = quote::quote! {
        #[derive(Clone)]
        pub struct #strct<#generic> {
            #fields
        }
    };

    let new = quote::quote! {
        impl<#generic> #strct<#generic> {
            pub const fn new(
                #fields_as_args
            ) -> Self {
                Self {
                    #fields_as_assign
                }
            }
        }
    };

    let collect = quote::quote! {
        inventory::collect!{
            #strct<fn() -> Box<dyn #trt>>
        }
    };

    let macro_def = quote::quote! {
        #[macro_export]
        macro_rules! #macro_name {
            (
                #fields_as_expr
            ) => {
                #macro_name!{
                    pub #handle {
                        #fields_as_expr_call
                    }
                }
            };
            (
                $vis: vis $plugin: ident {
                    #fields_as_expr
                }
            ) => {
                struct $plugin;

                fn get() -> Box<dyn #trt> {
                    Box::new($plugin)
                }

                inventory::submit!{
                    #strct::<fn() -> Box<dyn #trt>>::new(
                        #fields_as_expr_assign
                        get
                    )
                }
            }
        }
    };

    let plugin_collector = Ident::new(&format!("{}Collector", strct), Span::call_site());

    let plugin_collector = quote::quote! {
        pub struct #plugin_collector{
            pub plugins: Vec<#strct<Box<dyn #trt>>>
        }

        impl #plugin_collector {
            pub fn new() -> Self {
                Self::new_with(|_| {})
            }

            pub fn new_with<F: Fn(&mut #strct<fn() -> Box<dyn #trt>>)>(with: F) -> Self {
                let mut plugins = vec![];
                for plugin in inventory::iter::<#strct<fn() -> Box<dyn #trt>>> {
                    let mut plugin = plugin.clone();
                    with(&mut plugin);
                    plugins.push(#strct::new(
                        #field_as_reassign
                    ));
                }
                Self { plugins }
            }
        }
    };

    let out = quote::quote! {
        #struct_def

        #new

        #collect

        #macro_def

        #plugin_collector
    };

    // panic!("{}", out.to_string());

    out.into()
}
