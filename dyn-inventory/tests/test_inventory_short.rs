use dyn_inventory::dyn_inventory;

pub trait HandlesFlag {
    fn handle(&self);
}

dyn_inventory! {
    cFlag <Handle: HandlesFlag> {
        pub name: &'static str,
        handle: Handle
    };
}

mod my_flag {
    use dyn_inventory::emit;

    use crate::{FlagInit, HandlesFlag};

    emit! {
        Handle HandlesFlag as Flag {
            name = "my flag for abc-cli"
        }
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
