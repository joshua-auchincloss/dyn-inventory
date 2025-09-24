// invalid identifier in struct position (hyphen) for registry struct
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
    My-Handle<T: Plugin> {
        active: bool,
        t: T,
    };
);

fn main() {}
