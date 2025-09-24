// missing comma between two ExtraParams
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
    CommaIssueRegistry<T: PluginApi> {
        size: i16,
        t: T,
    };
    init_name = SomeInit
    unknown = RegisterTwo,
);

fn main() {}
