use dyn_inventory::dyn_inventory;

pub trait MyPlugin {
    fn value(&self) -> &'static str;
}

dyn_inventory! {
    Plugin<Handle: MyPlugin> {
        pub name: &'static str,
        pub desc: &'static str,
        handle: Handle
    };
}

mod my_plugin {
    use dyn_inventory::emit;

    use crate::{MyPlugin, PluginInit};

    emit! {
        Handle MyPlugin as Plugin {
            name = "my plugin for abc-framework",
            desc = "implements my plugin by doing xyz",
        }
    }

    impl MyPlugin for Handle {
        fn value(&self) -> &'static str {
            "some dynamic value"
        }
    }
}

#[test]
fn e2e() {
    let collected = PluginCollector::new();
    assert_eq!(collected.plugins.len(), 1);
    for plugin in &collected.plugins {
        assert_eq!(plugin.handle.value(), "some dynamic value");
        assert_eq!(plugin.name, "my plugin for abc-framework");
    }
}
