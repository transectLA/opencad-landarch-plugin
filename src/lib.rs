//! Land Arch Tools — Open CAD Studio landscape architecture plugin.
//!
//! Architecture follows the working Land Survey add-on:
//! Layer B (this cdylib) contains only host glue/ribbon/state; landscape-domain
//! rules live in the host-free `landarch-core` engine crate.

use ocs_plugin_api::host::{BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::CadModule;

mod cad;
mod commands;
mod dispatch;
mod ribbon;
mod state;
mod xdata;

pub const PLUGIN_ID: &str = "opencad.landarch";

static MANIFEST: PluginManifest = PluginManifest {
    id: PLUGIN_ID,
    name: "Land Arch Tools",
    version: env!("CARGO_PKG_VERSION"),
    description: "Landscape architecture planting, materials, furnishings, amenities, schedules, and QA/QC tools",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 60,
    xdata_apps: &[
        xdata::XDATA_PLANT,
        xdata::XDATA_PLANTING_AREA,
        xdata::XDATA_MATERIAL,
        xdata::XDATA_FURNISHING,
        xdata::XDATA_AMENITY,
    ],
    command_prefixes: &["LA_"],
};

struct LandArchPlugin;

impl BuiltinPlugin for LandArchPlugin {
    fn manifest(&self) -> &'static PluginManifest { &MANIFEST }
    fn ribbon(&self) -> Box<dyn CadModule> { Box::new(ribbon::LandArchModule) }
    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool { dispatch::handle(host, cmd) }
}

ocs_plugin_api::export_plugin!(LandArchPlugin);
