use std::collections::BTreeMap;
use acadrust::xdata::{ExtendedDataRecord, XDataValue};
use ocs_plugin_api::host::HostApi;
use crate::xdata;

#[derive(Default)]
struct Qty { qty:f64, unit:String, name:String, unit_cost:f64 }

fn record_from_entity<'a>(entity:&'a acadrust::EntityType,app:&str)->Option<&'a ExtendedDataRecord>{
    entity.common().extended_data.records.iter().find(|r|r.application_name==app)
}

pub fn plant_schedule(host:&mut dyn HostApi){
    let doc=host.document(); let mut counts=BTreeMap::<String,usize>::new();
    for e in doc.entities(){if let Some(r)=record_from_entity(e,xdata::XDATA_PLANT){let f=xdata::fields(r);if let Some(code)=f.get("code"){*counts.entry(code.clone()).or_default()+=1;}}}
    let mut out=String::from("LAND ARCH TOOLS — PLANT SCHEDULE\n\nCODE      BOTANICAL NAME                     COMMON NAME                  QTY\n--------------------------------------------------------------------------------\n");
    for (code,qty) in counts{if let Some(p)=landarch_core::catalog::plant(&code){out.push_str(&format!("{:<9} {:<34} {:<28} {:>4}\n",p.code,p.botanical,p.common,qty));}}
    host.push_output(&out);
}

pub fn site_schedule(host:&mut dyn HostApi){
    let doc=host.document(); let mut materials=BTreeMap::<String,Qty>::new(); let mut objects=BTreeMap::<String,Qty>::new();
    for e in doc.entities(){
        if let Some(r)=record_from_entity(e,xdata::XDATA_MATERIAL){let f=xdata::fields(r);if let Some(code)=f.get("code"){
            let q=f.get("quantity").and_then(|v|v.parse().ok()).unwrap_or(0.0); let cost=f.get("unit_cost").and_then(|v|v.parse().ok()).unwrap_or(0.0);
            let slot=materials.entry(code.clone()).or_default();slot.qty+=q;slot.unit=f.get("unit").cloned().unwrap_or_default();slot.name=f.get("name").cloned().unwrap_or_default();slot.unit_cost=cost;
        }}
        for app in [xdata::XDATA_FURNISHING,xdata::XDATA_AMENITY]{if let Some(r)=record_from_entity(e,app){let f=xdata::fields(r);if let Some(code)=f.get("code"){
            let slot=objects.entry(code.clone()).or_default();slot.qty+=1.0;slot.unit="EA".to_string();slot.name=f.get("name").cloned().unwrap_or_default();slot.unit_cost=f.get("unit_cost").and_then(|v|v.parse().ok()).unwrap_or(0.0);
        }}}
    }
    let mut out=String::from("LAND ARCH TOOLS — MATERIALS / FURNISHINGS / AMENITIES SCHEDULE\n\nMATERIALS\nCODE          DESCRIPTION                         QTY       UNIT       EST. COST\n--------------------------------------------------------------------------------\n");
    for (code,q) in materials{out.push_str(&format!("{:<13} {:<35} {:>9.2} {:<8} ${:>10.2}\n",code,q.name,q.qty,q.unit,q.qty*q.unit_cost));}
    out.push_str("\nFURNISHINGS + AMENITIES\nCODE          DESCRIPTION                         QTY       UNIT       EST. COST\n--------------------------------------------------------------------------------\n");
    for (code,q) in objects{out.push_str(&format!("{:<13} {:<35} {:>9.0} {:<8} ${:>10.2}\n",code,q.name,q.qty,q.unit,q.qty*q.unit_cost));}
    host.push_output(&out);
}

pub fn all_schedules(host:&mut dyn HostApi){plant_schedule(host);site_schedule(host);}

pub fn qc(host:&mut dyn HostApi){
    let doc=host.document(); let mut plants=0;let mut areas=0;let mut mats=0;let mut furn=0;let mut amenities=0;let mut bad=0;
    for e in doc.entities(){
        for (app,counter) in [(xdata::XDATA_PLANT,&mut plants),(xdata::XDATA_PLANTING_AREA,&mut areas),(xdata::XDATA_MATERIAL,&mut mats),(xdata::XDATA_FURNISHING,&mut furn),(xdata::XDATA_AMENITY,&mut amenities)]{
            if let Some(r)=record_from_entity(e,app){*counter+=1;let f=xdata::fields(r);if f.get("schema").is_none() && r.values.is_empty(){bad+=1;}}
        }
    }
    host.push_output(&format!("LAND ARCH TOOLS — QA/QC\nplants={plants}, planting areas={areas}, materials={mats}, furnishings={furn}, amenities={amenities}\nmalformed/empty LANDARCH records={bad}\nNote: v0.0.2 quantities on material areas/edges are captured at creation; geometry edits after placement require recreation until entity-change hooks are available."));
}
