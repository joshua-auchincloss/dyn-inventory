#![allow(clippy::needless_doctest_main)]
/* START OF README CONTENTS */

//! ## dyn-inventory
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/dyn-inventory?style=for-the-badge)](https://crates.io/crates/dyn-inventory)
//! [![docs.rs](https://img.shields.io/docsrs/dyn-inventory?style=for-the-badge)](https://docs.rs/dyn-inventory)
//! ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/joshua-auchincloss/dyn-inventory/test.yaml?style=for-the-badge&label=Tests)
//! ![Crates.io License](https://img.shields.io/crates/l/dyn-inventory?style=for-the-badge)
//!
//! proc macro for building runtime plugin registries using dyn-compatible traits and the inventory crate.
//!
//! this crate generates code to:
//!
//! - register plugins that implement a trait object (`dyn trait`)
//! - carry typed metadata alongside each plugin
//! - collect and instantiate all registered plugins at runtime
//!
//! ```rust
//! use dyn_inventory::dyn_inventory;
//!
//! pub trait MyPlugin {
//!     fn handle(&self);
//! }
//!
//! dyn_inventory! {
//!     Plugin<Handle: MyPlugin> {
//!         pub name: &'static str,
//!         desc: &'static str,
//!         handle: Handle
//!     }
//! }
//!
//! mod my_plugin {
//!     use crate::{MyPlugin, Plugin, PluginInit};
//!
//!     dyn_inventory::emit! {
//!         Handle MyPlugin as Plugin {
//!             name = "my plugin for abc-framework",
//!             desc = "implements my plugin by doing xyz"
//!         }
//!     }
//!
//!     impl MyPlugin for Handle {
//!         fn handle(&self) {
//!             println!("MyPlugin was used");
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let collected = PluginCollector::new();
//!     for plugin in &collected.plugins {
//!         plugin.handle.handle();
//!         // >> "MyPlugin was used"
//!     }
//! }
//! ```
//!
//! ## Why dyn-compatible traits
//!
//! the plugins produced by this crate are stored and used as `Box<dyn Trait>`. when used with [`inventory`](https://crates.io/crates/inventory), this allows for new plugin registries to be developed for decentralized libraries and frameworks.
//!
//! ## Quick Start
//!
//! 1. add dependencies:
//!
//! ```toml
//! [dependencies]
//! inventory = "0.3"
//! dyn-inventory = "0.1"
//! ```
//!
//! 2. define a trait that is dyn-compatible:
//!
//! ```rust
//! pub trait Greeter {
//!     fn greet(&self) -> String;
//! }
//! ```
//!
//! 3. declare your inventory using the `dyn_inventory!` proc macro:
//!
//! ```rust
//! pub trait Greeter {
//!     fn greet(&self) -> String;
//! }
//!
//! dyn_inventory::dyn_inventory!(
//!     GreeterPlugin<T: Greeter> {
//!         name: &'static str,
//!         version: u32,
//!         t: T,
//!     };
//! );
//! ```
//!
//!
//!  what this generates:
//!
//!  - a struct `GreeterPlugin` with the fields you declared, and a `Box<dyn Greeter>`
//!  - an inventory registration type `inventory::collect!(GreeterPluginInit)`
//!  - a collector `GreeterPluginCollector` that has `plugin` of type `Vec<GreeterPlugin>`
//!
//! 4. register a plugin somewhere in your code (could be another crate that depends on your trait crate):
//!
//! ```rust,ignore
//! use crate::{Greeter, GreeterPlugin, GreeterPluginInit};
//! use dyn_inventory::emit;
//!
//! // this expands to a unit struct named `MyGreeter` and registers it into the inventory
//! emit! {
//!     MyGreeter Greeter for GreeterPlugin {
//!         name = "hello",
//!         version = 1,
//!     }
//! }
//!
//! // you implement the trait for the generated unit struct
//! impl Greeter for MyGreeter {
//!     fn greet(&self) -> String { "hi".to_string() }
//! }
//! ```
//!
//! 5. collect your plugins at runtime:
//!
//! ```rust,ignore
//! let collected = GreeterPluginCollector::new();
//! for plugin in &collected.plugins {
//!     // `plugin.t` is now a `Box<dyn Greeter>`; other fields are your metadata
//!     println!("{} -> {}", plugin.name, plugin.t.greet());
//! }
//! ```
//!
//! ## Macro Syntax
//!
//! ```rust,ignore
//! use dyn_inventory::dyn_inventory;
//!
//! dyn_inventory!(
//!     // optional visibility specifier (pub | pub(crate))
//!     // StructName = the name of the struct that holds the Box<dyn TraitName>
//!     // TraitName - the trait which needs a dyn-inventory
//!     pub|pub(crate) StructName<Handle: TraitName> {
//!         // exactly one field must have type `Handle`.
//!         // the field whose type equals the generic parameter (`Generic`) is treated as the plugin “handle”.
//!         // internally during registration this field is filled with a function pointer `fn() -> Box<dyn TraitName>`, and the collector converts it to `Box<dyn TraitName>` by calling it.
//!         handle: Handle,
//!
//!         // optional visibity specifier
//!         // any number of metadata fields are preserved
//!         pub|pub(crate) field_name: &'static str,
//!         pub other_field: usize,
//!     };
//!     // optional, comma-separated extra params
//!     init_name = InitStructName,
//! );
//! ```
//!
//! ## Extra Parameters
//!
//! two extra params are currently accepted:
//!
//! - `init_name = ident`
//!   - sets the name of the generated initialization struct. by default it is the snake_case of `StructName` (for example, `GreeterPlugin` -> `greeter_plugin`).
//!
//! ## Advanced: customizing collection
//!
//! the collector type is named by appending `Collector` to your struct name. it exposes:
//!
//! - `new()` -> builds the collection without modification
//! - `new_with(|item: &mut StructName| {...})` -> allows you to mutate the raw entries after they are instantiated into `Box<dyn TraitName>`
//!
//! ## Constraints
//!
//! - your trait must be object-safe (dyn-compatible)
//! - the `inventory` crate must be linked into the final binary; ensure your plugin crates depend on `inventory` and your main binary pulls in the crates that perform registrations
//! - plugins must not carry state. instead, pass state as trait function parameters.
/* END OF README CONTENTS */

mod args;
mod declare;

use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::Ident;

use crate::{args::Args, declare::EmitArgs};

pub(crate) fn init(strct: &Ident) -> Ident {
    Ident::new(&format!("{strct}Init"), Span::call_site())
}

#[proc_macro]
pub fn dyn_inventory(tok: TokenStream) -> TokenStream {
    let args: Args = match syn::parse(tok) {
        Ok(parse) => parse,
        Err(e) => {
            return e.into_compile_error().into();
        },
    };

    quote::quote! { #args }.into()
}

#[proc_macro]
pub fn emit(tok: TokenStream) -> TokenStream {
    let emit: EmitArgs = match syn::parse2(tok.into()) {
        Ok(emit) => emit,
        Err(e) => return e.into_compile_error().into(),
    };

    quote::quote! { #emit }.into()
}
