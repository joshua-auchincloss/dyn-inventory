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
    Plugin<Handle: MyTrait> {
        handle: Handle
    };
}

fn main() {}
