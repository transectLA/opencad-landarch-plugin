use acadrust::{Circle, EntityType, Line, Text, Vector3};
use acadrust::xdata::ExtendedDataRecord;
use ocs_plugin_api::host::HostApi;

pub fn add_tagged(host: &mut dyn HostApi, mut ent: EntityType, layer: &str, rec: ExtendedDataRecord) {
    ent.common_mut().layer = layer.to_string();
    let handle = host.add_entity(ent);
    host.write_record(handle, rec);
    crate::state::mark_created();
}

pub fn add_circle(host: &mut dyn HostApi, x: f64, y: f64, radius: f64, layer: &str, rec: ExtendedDataRecord) {
    add_tagged(host, EntityType::Circle(Circle::from_coords(x, y, 0.0, radius)), layer, rec);
}

pub fn add_line(host: &mut dyn HostApi, x1: f64, y1: f64, x2: f64, y2: f64, layer: &str, rec: ExtendedDataRecord) {
    add_tagged(host, EntityType::Line(Line::from_points(Vector3::new(x1,y1,0.0), Vector3::new(x2,y2,0.0))), layer, rec);
}

pub fn add_text(host: &mut dyn HostApi, text: String, x: f64, y: f64, height: f64, layer: &str) {
    let mut ent = EntityType::Text(Text::with_value(text, Vector3::new(x,y,0.0)).with_height(height));
    ent.common_mut().layer = layer.to_string();
    host.add_entity(ent);
    crate::state::mark_created();
}

pub fn finish_mutation(host: &mut dyn HostApi) {
    host.bump_geometry();
    host.set_dirty();
}
