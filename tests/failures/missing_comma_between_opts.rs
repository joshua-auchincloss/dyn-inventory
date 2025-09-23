// missing comma between two ExtraParams
//
//
//
//
//
//
//
//
//s
use dyn_inventory::dyn_inventory;

dyn_inventory!(
    TraitA: StructA<T> {
        a: i16,
        t: T,
    };
    macro_name = my_macro
    handle_name = MyHandle,
);

fn main() {}
