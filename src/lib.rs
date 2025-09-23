#![allow(clippy::needless_doctest_main)]

/* START OF README CONTENTS */

/// ## dyn-inventory
///
/// procedural macro helpers for building a runtime plugin inventory around dyn-compatible traits using the `inventory` crate.
///
/// this crate generates code to:
///
/// - register plugins that implement a trait object (`dyn trait`)
/// - carry typed metadata alongside each plugin
/// - collect and instantiate all registered plugins at runtime
///
/// ```rust
/// use dyn_inventory::dyn_inventory;
///
/// pub trait MyPlugin {
///     fn handle(&self);
/// }
///
/// dyn_inventory! {
///     MyPlugin: Plugin<Handle> {
///         pub name: &'static str,
///         desc: &'static str,
///         handle: Handle
///     };
///     macro_name = new_plugin
/// }
///
/// mod my_plugin {
///     use crate::{MyPlugin, Plugin};
///
///     new_plugin! {
///         Handle {
///             name = "my plugin for abc-framework";
///             desc = "implements my plugin by doing xyz";
///         }
///     }
///
///     impl MyPlugin for Handle {
///         fn handle(&self) {
///             println!("MyPlugin was used");
///         }
///     }
/// }
///
/// fn main() {
///     let collected = PluginCollector::new();
///     for plugin in &collected.plugins {
///         plugin.handle.handle();
///         // >> "MyPlugin was used"
///     }
/// }
/// ```
///
/// ## Why dyn-compatible traits
///
/// the plugins produced by this crate are stored and used as `Box<dyn Trait>`. when used with [`inventory`](https://crates.io/crates/inventory), this allows for new plugin registries to be developed for decentralized libraries and frameworks.
///
/// ## Quick Start
///
/// 1. add dependencies:
///
/// ```toml
/// [dependencies]
/// inventory = "0.3"
/// dyn-inventory = "0.1"
/// ```
///
/// 2. define a trait that is dyn-compatible:
///
/// ```rust
/// pub trait Greeter {
///     fn greet(&self) -> String;
/// }
/// ```
///
/// 3. declare your inventory using the `dyn_inventory!` proc macro:
///
/// ```rust
/// pub trait Greeter {
///     fn greet(&self) -> String;
/// }
///
/// dyn_inventory::dyn_inventory!(
///     Greeter: GreeterPlugin<T> {
///         name: &'static str,
///         version: u32,
///         t: T,
///     };
///     // optional extra params, see below
///     macro_name = register_greeter,
/// );
/// ```
///
///
///  what this generates:
///
///  - a struct `GreeterPlugin<T>` with the fields you declared
///  - an implementation `impl<T> GreeterPlugin<T> { pub const fn new(...) -> Self }`
///  - an inventory registration type `inventory::collect!(GreeterPlugin<fn() -> Box<dyn Greeter>>)`
///  - a macro `register_greeter!` (snake_case of the struct name by default) to register plugins
///  - a collector `GreeterPluginCollector` that has `plugin` of type `Vec<GreeterPlugin<Box<dyn Greeter>>>`
///
/// 4. register a plugin somewhere in your code (could be another crate that depends on your trait crate):
///
/// ```rust,ignore
/// use crate::{Greeter, register_greeter};
///
/// // this expands to a unit struct named `MyGreeter` and registers it into the inventory
/// register_greeter! {
///     pub MyGreeter {
///         name = "hello";
///         version = 1;
///     }
/// }
///
/// // you implement the trait for the generated unit struct
/// impl Greeter for MyGreeter {
///     fn greet(&self) -> String { "hi".to_string() }
/// }
/// ```
///
/// 5. collect your plugins at runtime:
///
/// ```rust,ignore
/// let collected = GreeterPluginCollector::new();
/// for plugin in collected.plugins {
///     // `plugin.t` is now a `Box<dyn Greeter>`; other fields are your metadata
///     println!("{} -> {}", plugin.name, plugin.t.greet());
/// }
/// ```
///
/// ## Macro Syntax
///
/// ```rust,ignore
/// use dyn_inventory::dyn_inventory;
///
/// dyn_inventory!(
///     TraitName: StructName<Handle> {
///         // exactly one field must have type `Handle`.
///         // the field whose type equals the generic parameter (`Generic`) is treated as the plugin “handle”.
///         // internally during registration this field is filled with a function pointer `fn() -> Box<dyn TraitName>`, and the collector converts it to `Box<dyn TraitName>` by calling it.
///         handle: Handle,
///
///         // optional visibity specifier
///         // any number of metadata fields are preserved
///         pub|pub(crate)? field_name: &'static str,
///         pub other_field: usize,
///     };
///     // optional, comma-separated extra params
///     macro_name = some_ident,
///     handle_name = SomeIdent,
/// );
/// ```
///
/// ## Extra Parameters
///
/// two extra params are currently accepted:
///
/// - `macro_name = ident`
///   - sets the name of the generated registration macro. by default it is the snake_case of `StructName` (for example, `GreeterPlugin` -> `greeter_plugin`).
/// - `handle_name = Ident`
///   - sets the name of the generated handle which implements your plugin. (for example, `handle_name = TheImpl` requires `impl GreeterPlugin for TheImpl`)
///
/// ## advanced: customizing collection
///
/// the collector type is named by appending `Collector` to your struct name. it exposes:
///
/// - `new()` -> builds the collection without modification
/// - `new_with(|item: &mut StructName<fn() -> Box<dyn TraitName>>| {...})` -> allows you to mutate the raw entries before they are instantiated into `Box<dyn TraitName>`
///
/// ## limitations
///
/// - your trait must be object-safe (dyn-compatible)
/// - the `inventory` crate must be linked into the final binary; ensure your plugin crates depend on `inventory` and your main binary pulls in the crates that perform registrations
/* END OF README CONTENTS */
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
    let handle_name = &args.handle_name;
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
                    pub #handle_name {
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
