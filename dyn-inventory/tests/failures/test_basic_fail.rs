// unknown ExtraParams keyword should fail parsing
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
    UnknownKeyRegistry<T: PluginApi> {
        id: u32,
        t: T,
    };
    unknown_kw = Bogus,
);

fn main() {}
