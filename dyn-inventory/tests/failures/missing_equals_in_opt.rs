// missing equals sign in ExtraParams
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
    NoEqualsRegistry<T: PluginApi> {
        count: usize,
        t: T,
    };
    init_name bad_name,
);

fn main() {}
