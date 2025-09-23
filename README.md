## dyn-inventory

[![Crates.io Version](https://img.shields.io/crates/v/dyn-inventory?style=for-the-badge)](https://crates.io/crates/dyn-inventory)
[![docs.rs](https://img.shields.io/docsrs/dyn-inventory?style=for-the-badge)](https://docs.rs/dyn-inventory)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/joshua-auchincloss/dyn-inventory/test.yaml?style=for-the-badge&label=Tests)
![Crates.io License](https://img.shields.io/crates/l/dyn-inventory?style=for-the-badge)

proc macro for building runtime plugin registries using dyn-compatible traits and the inventory crate.

this crate generates code to:

- register plugins that implement a trait object (`dyn trait`)
- carry typed metadata alongside each plugin
- collect and instantiate all registered plugins at runtime

```rust
use dyn_inventory::dyn_inventory;

pub trait MyPlugin {
    fn handle(&self);
}

dyn_inventory! {
    MyPlugin: Plugin<Handle> {
        pub name: &'static str,
        desc: &'static str,
        handle: Handle
    };
    macro_name = new_plugin
}

mod my_plugin {
    use crate::{MyPlugin, Plugin};

    new_plugin! {
        Handle {
            name = "my plugin for abc-framework";
            desc = "implements my plugin by doing xyz";
        }
    }

    impl MyPlugin for Handle {
        fn handle(&self) {
            println!("MyPlugin was used");
        }
    }
}


fn main() {
    let collected = PluginCollector::new();
    for plugin in &collected.plugins {
        plugin.handle.handle();
        // >> "MyPlugin was used"
    }
}
```

## Why dyn-compatible traits

the plugins produced by this crate are stored and used as `Box<dyn Trait>`. when used with [`inventory`](https://crates.io/crates/inventory), this allows for new plugin registries to be developed for decentralized libraries and frameworks.

## Quick Start

1. add dependencies:

```toml
[dependencies]
inventory = "0.3"
dyn-inventory = "0.1"
```

2. define a trait that is dyn-compatible:

```rust
pub trait Greeter {
    fn greet(&self) -> String;
}
```

3. declare your inventory using the `dyn_inventory!` proc macro:

```rust
pub trait Greeter {
    fn greet(&self) -> String;
}

dyn_inventory::dyn_inventory!(
    Greeter: GreeterPlugin<T> {
        name: &'static str,
        version: u32,
        t: T,
    };
    // optional extra params, see below
    macro_name = register_greeter,
);
```

> [!TIP]
> what this generates:
>
> - a struct `GreeterPlugin<T>` with the fields you declared
> - an implementation `impl<T> GreeterPlugin<T> { pub const fn new(...) -> Self }`
> - an inventory registration type `inventory::collect!(GreeterPlugin<fn() -> Box<dyn Greeter>>)`
> - a macro `register_greeter!` (snake_case of the struct name by default) to register plugins
> - a collector `GreeterPluginCollector` that has `plugin` of type `Vec<GreeterPlugin<Box<dyn Greeter>>>`

4. register a plugin somewhere in your code (could be another crate that depends on your trait crate):

```rust,ignore
use crate::{Greeter, register_greeter};

// this expands to a unit struct named `MyGreeter` and registers it into the inventory
register_greeter! {
    pub MyGreeter {
        name = "hello";
        version = 1;
    }
}

// you implement the trait for the generated unit struct
impl Greeter for MyGreeter {
    fn greet(&self) -> String { "hi".to_string() }
}
```

5. collect your plugins at runtime:

```rust,ignore
let collected = GreeterPluginCollector::new();
for plugin in collected.plugins {
    // `plugin.t` is now a `Box<dyn Greeter>`; other fields are your metadata
    println!("{} -> {}", plugin.name, plugin.t.greet());
}
```

## Macro Syntax

```rust,ignore
use dyn_inventory::dyn_inventory;

dyn_inventory!(
    TraitName: StructName<Handle> {
        // exactly one field must have type `Handle`.
        // the field whose type equals the generic parameter (`Generic`) is treated as the plugin “handle”.
        // internally during registration this field is filled with a function pointer `fn() -> Box<dyn TraitName>`, and the collector converts it to `Box<dyn TraitName>` by calling it.
        handle: Handle,

        // optional visibity specifier
        // any number of metadata fields are preserved
        pub|pub(crate)? field_name: &'static str,
        pub other_field: usize,
    };
    // optional, comma-separated extra params
    macro_name = some_ident,
    handle_name = SomeIdent,
);
```

## Extra Parameters

two extra params are currently accepted:

- `macro_name = ident`
  - sets the name of the generated registration macro. by default it is the snake_case of `StructName` (for example, `GreeterPlugin` -> `greeter_plugin`).
- `handle_name = Ident`
  - sets the name of the generated handle which implements your plugin. (for example, `handle_name = TheImpl` requires `impl GreeterPlugin for TheImpl`)

## Advanced: customizing collection

the collector type is named by appending `Collector` to your struct name. it exposes:

- `new()` -> builds the collection without modification
- `new_with(|item: &mut StructName<fn() -> Box<dyn TraitName>>| {...})` -> allows you to mutate the raw entries before they are instantiated into `Box<dyn TraitName>`

## Constraints

- your trait must be object-safe (dyn-compatible)
- the `inventory` crate must be linked into the final binary; ensure your plugin crates depend on `inventory` and your main binary pulls in the crates that perform registrations
