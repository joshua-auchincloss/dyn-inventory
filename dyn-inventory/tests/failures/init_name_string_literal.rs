// macro_name extraparams value must be an identifier not string literal
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
    LiteralRegistry<T: Plugin> {
        data: u64,
        t: T,
    };
    init_name = "MyInit",
);

fn main() {}
