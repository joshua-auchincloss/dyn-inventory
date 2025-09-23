// trait is not dyn compatible
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

trait MyTrait {
    fn nolookup() -> &'static str;
}

dyn_inventory! {
    MyTrait: Plugin<Handle> {
        handle: Handle
    };
}

fn main() {}
