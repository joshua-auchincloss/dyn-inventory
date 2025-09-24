// using a reserved keyword as init_name should not parse as Ident in this position
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
    StructC<T: TraitC> {
        a: char,
        t: T,
    };
    init_name = fn,
);

fn main() {}
