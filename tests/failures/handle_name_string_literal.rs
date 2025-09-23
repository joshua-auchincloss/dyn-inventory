// handle_name must be an identifier
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
    MyTrait: MyStruct<T> {
        data: u64,
        t: T,
    };
    handle_name = "HandleThing",
);

fn main() {}
