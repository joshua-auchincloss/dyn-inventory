use dyn_inventory::dyn_inventory;

pub trait MyPlugin {
    fn value(&self) -> &'static str;
}

dyn_inventory! {
    MyPlugin: Plugin<Handle> {
        pub name: &'static str,
        desc: &'static str,
        handle: Handle
    };
    macro_name = new_plugin
}

mod my_plugin {
    use crate::{MyPlugin, Plugin};

    new_plugin! {
        Handle {
            name = "my plugin for abc-framework";
            desc = "implements my plugin by doing xyz";
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
