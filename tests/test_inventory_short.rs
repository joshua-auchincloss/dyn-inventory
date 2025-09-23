use dyn_inventory::dyn_inventory;

pub trait HandlesFlag {
    fn handle(&self);
}

dyn_inventory! {
    HandlesFlag: Flag<Handle> {
        pub name: &'static str,
        handle: Handle
    };
    macro_name = new_flag,
    handle_name = Handle
}

mod my_flag {
    use crate::{Flag, HandlesFlag};

    new_flag! {
        name = "my flag for abc-cli";
    }

    impl HandlesFlag for Handle {
        fn handle(&self) {}
    }
}

#[test]
fn e2e() {
    let collected = FlagCollector::new();
    assert_eq!(collected.plugins.len(), 1);
    for plugin in &collected.plugins {
        plugin.handle.handle();
        assert_eq!(plugin.name, "my flag for abc-cli");
    }
}
