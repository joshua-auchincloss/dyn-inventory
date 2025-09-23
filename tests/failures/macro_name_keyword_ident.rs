// using a reserved keyword as macro_name should not parse as Ident in this position
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
    TraitC: StructC<T> {
        a: char,
        t: T,
    };
    macro_name = fn,
);

fn main() {}
