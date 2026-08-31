use ocs_plugin_api::host::{BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

// Keep these in sync with plugin.toml.
static MANIFEST: PluginManifest = PluginManifest {
    id: "opencad.landarch",
    name: "Land Arch Tools",
    version: "0.1.0",
    description: "Landscape architecture design and construction-document tools for Open CAD Studio",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 60,
    xdata_apps: &["LANDARCH"],
    command_prefixes: &["LA_"],
};

/// The "Land Arch Tools" ribbon tab. Starts with one placeholder command
/// (LA_HELLO) so we can confirm the plugin loads and dispatches before
/// building out plant tagging / schedule generation.
struct LandArchModule;

impl CadModule for LandArchModule {
    fn id(&self) -> &'static str {
        "landarch"
    }

    fn title(&self) -> &'static str {
        "Land Arch"
    }

    fn ribbon_groups(&self) -> Vec<RibbonGroup> {
        vec![RibbonGroup {
            title: "Getting Started",
            tools: vec![RibbonItem::LargeTool(ToolDef {
                id: "LA_HELLO",
                label: "Hello",
                icon: IconKind::Glyph("🌿"),
                event: ModuleEvent::Command("LA_HELLO".to_string()),
            })],
        }]
    }
}

struct LandArchPlugin;

impl BuiltinPlugin for LandArchPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }

    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(LandArchModule)
    }

    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        match cmd {
            "LA_HELLO" => {
                host.push_info("Land Arch Tools is alive.");
                true
            }
            _ => false,
        }
    }
}

ocs_plugin_api::export_plugin!(LandArchPlugin);
