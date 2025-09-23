// missing semicolon before ExtraParams list
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
    Tr: St<T> {
        x: i32,
        t: T,
    }
    macro_name = my_macro,
);

fn main() {}
