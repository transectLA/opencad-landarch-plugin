//! Land Arch Tools ribbon definition.
//!
//! OpenCADStudio currently permits one `CadModule` (one ribbon tab) per plugin.
//! Materials / Furnishings / Amenities are therefore a clearly separated second
//! ribbon area within the same Land Arch Tools tab. The functions are isolated
//! so they can be promoted to a separate module if multi-ribbon plugins arrive.

use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

pub struct LandArchModule;

impl CadModule for LandArchModule {
    fn id(&self) -> &'static str { "landarch" }
    fn title(&self) -> &'static str { "Land Arch Tools" }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            let mut groups = planting_groups();
            groups.extend(site_groups());
            groups.extend(documentation_groups());
            groups
        })
    }
}

fn cmd(id: &'static str, label: &'static str, glyph: &'static str, command: &'static str) -> RibbonItem {
    RibbonItem::LargeTool(ToolDef {
        id,
        label,
        icon: IconKind::Glyph(glyph),
        event: ModuleEvent::Command(command.to_string()),
    })
}

fn planting_groups() -> Vec<RibbonGroup> {
    vec![
        RibbonGroup {
            title: "Project",
            tools: vec![
                cmd("LA_HELP", "Help", "?", "LA_HELP"),
                cmd("LA_STATUS", "Status", "i", "LA_STATUS"),
            ],
        },
        RibbonGroup {
            title: "Planting",
            tools: vec![
                cmd("LA_PLANTS", "Plant Catalog", "P", "LA_PLANTS"),
                cmd("LA_PALETTES", "Palettes", "C", "LA_PALETTES"),
                cmd("LA_TREE", "Tree", "T", "LA_TREE QAGR"),
                cmd("LA_SHRUB", "Shrub / Grass", "S", "LA_SHRUB RHIN"),
                cmd("LA_AREA", "Planting Area", "A", "LA_AREA NATIVE 4"),
            ],
        },
        RibbonGroup {
            title: "Plant Layout",
            tools: vec![
                cmd("LA_GRID", "Grid", "#", "LA_GRID LOMI 0 0 30 20 3"),
                cmd("LA_NATURALIZE", "Naturalize", "*", "LA_NATURALIZE MUCA 0 0 30 20 3"),
                cmd("LA_LABEL", "Plant Label", "L", "LA_LABEL QAGR"),
            ],
        },
    ]
}

/// Second ribbon area requested for site materials and site elements.
fn site_groups() -> Vec<RibbonGroup> {
    vec![
        RibbonGroup {
            title: "Materials",
            tools: vec![
                cmd("LA_MATERIALS", "Material Catalog", "M", "LA_MATERIALS"),
                cmd("LA_MAT_AREA", "Material Area", "A", "LA_MAT_AREA PAV-CONC"),
                cmd("LA_MAT_EDGE", "Edge / Wall", "E", "LA_MAT_EDGE EDGE-STEEL"),
            ],
        },
        RibbonGroup {
            title: "Furnishings",
            tools: vec![
                cmd("LA_FURNISHINGS", "Catalog", "F", "LA_FURNISHINGS"),
                cmd("LA_FURN", "Place Furnishing", "B", "LA_FURN BENCH-01"),
            ],
        },
        RibbonGroup {
            title: "Amenities",
            tools: vec![
                cmd("LA_AMENITIES", "Catalog", "A", "LA_AMENITIES"),
                cmd("LA_AMENITY", "Place Amenity", "+", "LA_AMENITY DRINK-01"),
            ],
        },
    ]
}

fn documentation_groups() -> Vec<RibbonGroup> {
    vec![RibbonGroup {
        title: "Documentation",
        tools: vec![
            cmd("LA_PLANT_SCHEDULE", "Plant Schedule", "1", "LA_PLANT_SCHEDULE"),
            cmd("LA_SITE_SCHEDULE", "Site Schedule", "2", "LA_SITE_SCHEDULE"),
            cmd("LA_SCHEDULE", "All Schedules", "3", "LA_SCHEDULE"),
            cmd("LA_QC", "QA / QC", "!", "LA_QC"),
        ],
    }]
}
