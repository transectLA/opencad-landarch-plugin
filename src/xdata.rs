use std::collections::BTreeMap;
use acadrust::xdata::{ExtendedDataRecord, XDataValue};

pub const XDATA_PLANT: &str = "LANDARCH_PLANT";
pub const XDATA_PLANTING_AREA: &str = "LANDARCH_PLANTING_AREA";
pub const XDATA_MATERIAL: &str = "LANDARCH_MATERIAL";
pub const XDATA_FURNISHING: &str = "LANDARCH_FURNISHING";
pub const XDATA_AMENITY: &str = "LANDARCH_AMENITY";

pub fn record(app: &str, fields: &[(&str, String)]) -> ExtendedDataRecord {
    let mut r = ExtendedDataRecord::new(app);
    r.add_value(XDataValue::String("schema=2".to_string()));
    for (k, v) in fields {
        r.add_value(XDataValue::String(format!("{k}={v}")));
    }
    r
}

pub fn fields(rec: &ExtendedDataRecord) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for value in &rec.values {
        if let XDataValue::String(s) = value {
            if let Some((k, v)) = s.split_once('=') {
                out.insert(k.to_string(), v.to_string());
            }
        }
    }
    out
}
