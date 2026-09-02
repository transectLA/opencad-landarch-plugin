use acadrust::{Circle, EntityType, LwPolyline, Text, Vector2, Vector3};
use ocs_plugin_api::host::{CommandStep, HostApi, InteractiveCommand};
use landarch_core::{catalog, geometry};
use crate::{cad, xdata};

pub fn handle(host: &mut dyn HostApi, verb: &str, cmd: &str) -> bool {
    match verb {
        "LA_PLANTS" => list_plants(host),
        "LA_PALETTES" => list_palettes(host),
        "LA_TREE" => place_plant(host, cmd, true),
        "LA_SHRUB" => place_plant(host, cmd, false),
        "LA_AREA" => planting_area(host, cmd),
        "LA_GRID" => populate(host, cmd, false),
        "LA_NATURALIZE" => populate(host, cmd, true),
        "LA_LABEL" => label(host, cmd),
        "LA_PLANT_SCHEDULE" => crate::commands::reports::plant_schedule(host),
        _ => return false,
    }
    true
}

fn arg(cmd: &str, n: usize) -> Option<&str> { cmd.split_whitespace().nth(n + 1) }
fn num(s: &str) -> Result<f64, String> { s.parse::<f64>().ok().filter(|v| v.is_finite()).ok_or_else(|| format!("'{s}' is not a finite number")) }

fn list_plants(host: &mut dyn HostApi) {
    host.push_output(&landarch_core::format_plant_catalog());
}
fn list_palettes(host: &mut dyn HostApi) {
    host.push_output(&landarch_core::format_palette_catalog());
}

fn place_plant(host: &mut dyn HostApi, cmd: &str, require_tree: bool) {
    let default = if require_tree { "QAGR" } else { "RHIN" };
    let code = arg(cmd,0).unwrap_or(default);
    let Some(p) = catalog::plant(code) else { host.push_error(&format!("Unknown plant code {code}. Use LA_PLANTS.")); return; };
    if require_tree && p.category != landarch_core::PlantCategory::Tree { host.push_error(&format!("{} is not a tree.", p.code)); return; }
    if !require_tree && p.category == landarch_core::PlantCategory::Tree { host.push_error(&format!("{} is a tree; use LA_TREE.", p.code)); return; }
    crate::state::state().last_plant = p.code.to_string();
    host.push_undo(if require_tree { "LA_TREE" } else { "LA_SHRUB" });
    host.start_interactive(Box::new(PlacePlant { plant: p }));
}

struct PlacePlant { plant: &'static landarch_core::Plant }
impl InteractiveCommand for PlacePlant {
    fn prompt(&self) -> String { format!("Place {} ({}) — specify insertion point", self.plant.common, self.plant.code) }
    fn on_point(&mut self, pt: [f64;3]) -> CommandStep {
        let mut c = Circle::from_coords(pt[0],pt[1],pt[2],self.plant.symbol_radius);
        c.common.layer = self.plant.layer.to_string();
        c.common.extended_data.upsert_record(xdata::record(xdata::XDATA_PLANT, &[
            ("object_type", "plant".to_string()),
            ("code", self.plant.code.to_string()),
            ("botanical", self.plant.botanical.to_string()),
            ("common", self.plant.common.to_string()),
            ("category", self.plant.category.as_str().to_string()),
            ("spacing", format!("{:.3}", self.plant.default_spacing)),
            ("role", "individual".to_string()),
        ]));
        CommandStep::CommitAndEnd(EntityType::Circle(c))
    }
}

fn planting_area(host: &mut dyn HostApi, cmd: &str) {
    let pname = arg(cmd,0).unwrap_or("NATIVE");
    let spacing = match arg(cmd,1) { Some(v)=>match num(v){Ok(v) if v>0.0=>v,_=>{host.push_error("LA_AREA spacing must be > 0.");return;}}, None=>4.0 };
    let Some(pal) = catalog::palette(pname) else { host.push_error(&format!("Unknown palette {pname}. Use LA_PALETTES.")); return; };
    let id = format!("PA-{:04}", host.document().entities().count()+1);
    host.push_undo("LA_AREA");
    host.start_interactive(Box::new(PlaceArea{palette:pal,spacing,first:None,id}));
}

struct PlaceArea { palette: &'static landarch_core::Palette, spacing:f64, first:Option<[f64;3]>, id:String }
impl InteractiveCommand for PlaceArea {
    fn prompt(&self)->String { if self.first.is_none(){format!("{} — first corner",self.id)}else{"Opposite corner".to_string()} }
    fn on_point(&mut self,pt:[f64;3])->CommandStep {
        if self.first.is_none(){self.first=Some(pt);return CommandStep::NeedPoint;}
        let a=self.first.unwrap(); let b=pt;
        let pts=vec![Vector2::new(a[0],a[1]),Vector2::new(b[0],a[1]),Vector2::new(b[0],b[1]),Vector2::new(a[0],b[1]),Vector2::new(a[0],a[1])];
        let mut pl=LwPolyline::from_points(pts);
        pl.common.layer="L-PLNT-AREA".to_string();
        let area=geometry::rectangle_area(a[0],a[1],b[0],b[1]);
        pl.common.extended_data.upsert_record(xdata::record(xdata::XDATA_PLANTING_AREA,&[
            ("object_type","planting_area".to_string()),("area_id",self.id.clone()),("palette",self.palette.name.to_string()),
            ("spacing",format!("{:.3}",self.spacing)),("area",format!("{area:.3}"))
        ]));
        CommandStep::CommitAndEnd(EntityType::LwPolyline(pl))
    }
}

fn populate(host:&mut dyn HostApi,cmd:&str,naturalized:bool){
    let usage=if naturalized{"Usage: LA_NATURALIZE CODE X1 Y1 X2 Y2 SPACING"}else{"Usage: LA_GRID CODE X1 Y1 X2 Y2 SPACING"};
    let a:Vec<&str>=cmd.split_whitespace().skip(1).collect(); if a.len()!=6{host.push_info(usage);return;}
    let Some(p)=catalog::plant(a[0]) else{host.push_error(&format!("Unknown plant {}",a[0]));return;};
    let mut n=[0.0;5]; for i in 0..5{match num(a[i+1]){Ok(v)=>n[i]=v,Err(e)=>{host.push_error(&format!("{e}. {usage}"));return;}}}
    if n[4]<=0.0{host.push_error("Spacing must be > 0.");return;}
    let pts=if naturalized{geometry::naturalized_points(n[0],n[1],n[2],n[3],n[4])}else{geometry::grid_points(n[0],n[1],n[2],n[3],n[4])};
    host.push_undo(if naturalized{"LA_NATURALIZE"}else{"LA_GRID"});
    for (x,y) in &pts{
        cad::add_circle(host,*x,*y,p.symbol_radius,p.layer,xdata::record(xdata::XDATA_PLANT,&[
            ("object_type","plant".to_string()),("code",p.code.to_string()),("botanical",p.botanical.to_string()),("common",p.common.to_string()),
            ("category",p.category.as_str().to_string()),("spacing",format!("{:.3}",n[4])),("role",if naturalized{"naturalized".to_string()}else{"grid".to_string()})
        ]));
    }
    cad::finish_mutation(host); crate::state::state().last_plant=p.code.to_string();
    host.push_output(&format!("{}: placed {} {} at nominal spacing {:.3}.",if naturalized{"LA_NATURALIZE"}else{"LA_GRID"},pts.len(),p.code,n[4]));
}

fn label(host:&mut dyn HostApi,cmd:&str){
    let code=arg(cmd,0).unwrap_or("QAGR"); let Some(p)=catalog::plant(code) else{host.push_error(&format!("Unknown plant {code}"));return;};
    host.push_undo("LA_LABEL"); host.start_interactive(Box::new(PlacePlantLabel{plant:p}));
}
struct PlacePlantLabel{plant:&'static landarch_core::Plant}
impl InteractiveCommand for PlacePlantLabel{
    fn prompt(&self)->String{format!("Place label for {} — specify text insertion point",self.plant.code)}
    fn on_point(&mut self,pt:[f64;3])->CommandStep{
        let mut t=Text::with_value(format!("{} — {}",self.plant.code,self.plant.common),Vector3::new(pt[0],pt[1],pt[2])).with_height(1.5);
        t.common.layer="L-ANNO-PLNT".to_string(); CommandStep::CommitAndEnd(EntityType::Text(t))
    }
}
