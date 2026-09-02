//! LA_* command router. Domain calculations remain in `landarch-core`.

use ocs_plugin_api::host::HostApi;

pub fn handle(host: &mut dyn HostApi, cmd: &str) -> bool {
    let verb = cmd.split_whitespace().next().unwrap_or("").to_uppercase();
    if !verb.starts_with("LA_") { return false; }

    match verb.as_str() {
        "LA_HELP" => { help(host); true }
        "LA_STATUS" => { status(host); true }
        "LA_PLANTS" | "LA_PALETTES" | "LA_TREE" | "LA_SHRUB" | "LA_AREA" |
        "LA_GRID" | "LA_NATURALIZE" | "LA_LABEL" | "LA_PLANT_SCHEDULE" =>
            crate::commands::planting::handle(host, &verb, cmd),
        "LA_MATERIALS" | "LA_MAT_AREA" | "LA_MAT_EDGE" | "LA_FURNISHINGS" |
        "LA_FURN" | "LA_AMENITIES" | "LA_AMENITY" | "LA_SITE_SCHEDULE" =>
            crate::commands::site::handle(host, &verb, cmd),
        "LA_SCHEDULE" => { crate::commands::reports::all_schedules(host); true }
        "LA_QC" => { crate::commands::reports::qc(host); true }
        _ => { host.push_error(&format!("Land Arch Tools: unknown command {verb}. Use LA_HELP.")); true }
    }
}

fn help(host: &mut dyn HostApi) {
    host.push_info(
        "Land Arch Tools v0.0.2 — Planting: LA_PLANTS, LA_PALETTES, LA_TREE, LA_SHRUB, LA_AREA, LA_GRID, LA_NATURALIZE, LA_LABEL, LA_PLANT_SCHEDULE. Materials/site: LA_MATERIALS, LA_MAT_AREA, LA_MAT_EDGE, LA_FURNISHINGS, LA_FURN, LA_AMENITIES, LA_AMENITY, LA_SITE_SCHEDULE. Documentation: LA_SCHEDULE, LA_QC. See PLUGIN.md."
    );
}

fn status(host: &mut dyn HostApi) {
    let st = crate::state::state();
    host.push_output(&format!(
        "Land Arch Tools v{} | last plant={} | material={} | furnishing={} | amenity={} | created this session={}",
        env!("CARGO_PKG_VERSION"), st.last_plant, st.last_material, st.last_furnishing, st.last_amenity, st.created_this_session
    ));
}
