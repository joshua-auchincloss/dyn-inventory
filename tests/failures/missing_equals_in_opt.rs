// missing '=' between keyword and value
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
    Tt: Ss<T> {
        a: usize,
        t: T,
    };
    macro_name my_macro,
);

fn main() {}
