// missing semicolon before ExtraParams
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
    MissingSemiRegistry<T: PluginApi> {
        value: i32,
        t: T,
    }
    init_name = register_missing,
);

fn main() {}
