// macro_name must be an identifier, not a string literal
//
//
//
//
//
//
//
//
//
use dyn_inventory::dyn_inventory;

dyn_inventory!(
    TraitY: PluginY<T> {
        pub a: u8,
        t: T,
    };
    macro_name = "my_macro",
);

fn main() {}
