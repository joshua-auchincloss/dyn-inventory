// uses unknown ExtraParams keyword to trigger parser error
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
    TraitX: PluginX<T> {
        a: u32,
        t: T,
    };
    unknown_kw = Foo,
);

fn main() {}
