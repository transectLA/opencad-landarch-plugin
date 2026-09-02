use acadrust::{Circle, EntityType, Line, LwPolyline, Vector2, Vector3};
use ocs_plugin_api::host::{CommandStep, HostApi, InteractiveCommand};
use landarch_core::{catalog, geometry, SiteCategory, Unit};
use crate::xdata;

pub fn handle(host:&mut dyn HostApi,verb:&str,cmd:&str)->bool{
    match verb{
        "LA_MATERIALS"=>host.push_output(&landarch_core::format_material_catalog()),
        "LA_MAT_AREA"=>material_area(host,cmd),
        "LA_MAT_EDGE"=>material_edge(host,cmd),
        "LA_FURNISHINGS"=>host.push_output(&landarch_core::format_furnishing_catalog()),
        "LA_FURN"=>place_site_object(host,cmd,SiteCategory::Furnishing),
        "LA_AMENITIES"=>host.push_output(&landarch_core::format_amenity_catalog()),
        "LA_AMENITY"=>place_site_object(host,cmd,SiteCategory::Amenity),
        "LA_SITE_SCHEDULE"=>crate::commands::reports::site_schedule(host),
        _=>return false,
    } true
}
fn arg(cmd:&str,n:usize)->Option<&str>{cmd.split_whitespace().nth(n+1)}

fn material_area(host:&mut dyn HostApi,cmd:&str){
    let code=arg(cmd,0).unwrap_or("PAV-CONC"); let Some(m)=catalog::material(code) else{host.push_error(&format!("Unknown material {code}. Use LA_MATERIALS."));return;};
    if m.unit!=Unit::SquareFeet{host.push_error(&format!("{} is measured in {}; use LA_MAT_EDGE for linear materials.",m.code,m.unit.as_str()));return;}
    crate::state::state().last_material=m.code.to_string(); host.push_undo("LA_MAT_AREA"); host.start_interactive(Box::new(PlaceMaterialArea{material:m,first:None}));
}
struct PlaceMaterialArea{material:&'static landarch_core::Material,first:Option<[f64;3]>}
impl InteractiveCommand for PlaceMaterialArea{
    fn prompt(&self)->String{if self.first.is_none(){format!("{} — first corner",self.material.code)}else{"Opposite corner".to_string()}}
    fn on_point(&mut self,pt:[f64;3])->CommandStep{
        if self.first.is_none(){self.first=Some(pt);return CommandStep::NeedPoint;}
        let a=self.first.unwrap(); let b=pt; let area=geometry::rectangle_area(a[0],a[1],b[0],b[1]);
        let pts=vec![Vector2::new(a[0],a[1]),Vector2::new(b[0],a[1]),Vector2::new(b[0],b[1]),Vector2::new(a[0],b[1]),Vector2::new(a[0],a[1])];
        let mut pl=LwPolyline::from_points(pts); pl.common.layer=self.material.layer.to_string();
        pl.common.extended_data.upsert_record(xdata::record(xdata::XDATA_MATERIAL,&[
            ("object_type","material".to_string()),("code",self.material.code.to_string()),("name",self.material.name.to_string()),
            ("category",self.material.category.as_str().to_string()),("unit",self.material.unit.as_str().to_string()),("quantity",format!("{area:.3}")),
            ("spec",self.material.spec_section.to_string()),("detail",self.material.detail.to_string()),("unit_cost",format!("{:.2}",self.material.unit_cost))
        ])); CommandStep::CommitAndEnd(EntityType::LwPolyline(pl))
    }
}

fn material_edge(host:&mut dyn HostApi,cmd:&str){
    let code=arg(cmd,0).unwrap_or("EDGE-STEEL"); let Some(m)=catalog::material(code) else{host.push_error(&format!("Unknown material {code}"));return;};
    if m.unit!=Unit::LinearFeet{host.push_error(&format!("{} is measured in {}; use LA_MAT_AREA for area materials.",m.code,m.unit.as_str()));return;}
    crate::state::state().last_material=m.code.to_string(); host.push_undo("LA_MAT_EDGE"); host.start_interactive(Box::new(PlaceMaterialEdge{material:m,first:None}));
}
struct PlaceMaterialEdge{material:&'static landarch_core::Material,first:Option<[f64;3]>}
impl InteractiveCommand for PlaceMaterialEdge{
    fn prompt(&self)->String{if self.first.is_none(){format!("{} — start point",self.material.code)}else{"End point".to_string()}}
    fn on_point(&mut self,pt:[f64;3])->CommandStep{
        if self.first.is_none(){self.first=Some(pt);return CommandStep::NeedPoint;} let a=self.first.unwrap(); let q=geometry::distance(a[0],a[1],pt[0],pt[1]);
        let mut l=Line::from_points(Vector3::new(a[0],a[1],a[2]),Vector3::new(pt[0],pt[1],pt[2])); l.common.layer=self.material.layer.to_string();
        l.common.extended_data.upsert_record(xdata::record(xdata::XDATA_MATERIAL,&[
            ("object_type","material".to_string()),("code",self.material.code.to_string()),("name",self.material.name.to_string()),
            ("category",self.material.category.as_str().to_string()),("unit",self.material.unit.as_str().to_string()),("quantity",format!("{q:.3}")),
            ("spec",self.material.spec_section.to_string()),("detail",self.material.detail.to_string()),("unit_cost",format!("{:.2}",self.material.unit_cost))
        ])); CommandStep::CommitAndEnd(EntityType::Line(l))
    }
}

fn place_site_object(host:&mut dyn HostApi,cmd:&str,category:SiteCategory){
    let default=if category==SiteCategory::Furnishing{"BENCH-01"}else{"DRINK-01"}; let code=arg(cmd,0).unwrap_or(default);
    let obj=if category==SiteCategory::Furnishing{catalog::furnishing(code)}else{catalog::amenity(code)};
    let Some(obj)=obj else{host.push_error(&format!("Unknown {} code {code}",category.as_str()));return;};
    if category==SiteCategory::Furnishing{crate::state::state().last_furnishing=obj.code.to_string()}else{crate::state::state().last_amenity=obj.code.to_string()}
    host.push_undo(if category==SiteCategory::Furnishing{"LA_FURN"}else{"LA_AMENITY"}); host.start_interactive(Box::new(PlaceSiteObject{object:obj}));
}
struct PlaceSiteObject{object:&'static landarch_core::SiteObject}
impl InteractiveCommand for PlaceSiteObject{
    fn prompt(&self)->String{format!("Place {} ({}) — specify insertion point",self.object.name,self.object.code)}
    fn on_point(&mut self,pt:[f64;3])->CommandStep{
        let mut c=Circle::from_coords(pt[0],pt[1],pt[2],self.object.symbol_radius); c.common.layer=self.object.layer.to_string();
        let app=if self.object.category==SiteCategory::Furnishing{xdata::XDATA_FURNISHING}else{xdata::XDATA_AMENITY};
        c.common.extended_data.upsert_record(xdata::record(app,&[
            ("object_type",self.object.category.as_str().to_lowercase()),("code",self.object.code.to_string()),("name",self.object.name.to_string()),
            ("manufacturer",self.object.manufacturer.to_string()),("model",self.object.model.to_string()),("unit_cost",format!("{:.2}",self.object.unit_cost)),
            ("spec",self.object.spec_section.to_string()),("detail",self.object.detail.to_string())
        ])); CommandStep::CommitAndEnd(EntityType::Circle(c))
    }
}
