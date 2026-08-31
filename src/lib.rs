//! Land Arch Tools v0.0.2
//! Planting Designer vertical slice for Open CAD Studio.

use std::collections::BTreeMap;

use ocs_plugin_api::host::{
    acadrust::{
        entities::{
            Circle, EntityType, LwPolyline, Text,
            TextHorizontalAlignment, TextVerticalAlignment,
        },
        types::{Vector2, Vector3},
        xdata::{ExtendedDataRecord, XDataValue},
    },
    BuiltinPlugin, CommandStep, Handle, HostApi, InteractiveCommand,
};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{
    CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef,
};

const XDATA_APP: &str = "LANDARCH";

static MANIFEST: PluginManifest = PluginManifest {
    id: "opencad.landarch",
    name: "Land Arch Tools",
    version: "0.0.2",
    description: "Landscape architecture planting design and construction documentation tools for Open CAD Studio",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 70,
    xdata_apps: &[XDATA_APP],
    command_prefixes: &["LA_"],
};

#[derive(Clone, Copy)]
struct Plant {
    code: &'static str,
    botanical: &'static str,
    common: &'static str,
    category: &'static str,
    default_radius: f64,
    default_spacing: f64,
    palette: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant { code:"QAGR", botanical:"Quercus agrifolia", common:"Coast Live Oak", category:"Tree", default_radius:4.0, default_spacing:25.0, palette:"NATIVE" },
    Plant { code:"ARUB", botanical:"Arbutus unedo", common:"Strawberry Tree", category:"Tree", default_radius:3.0, default_spacing:18.0, palette:"NATIVE" },
    Plant { code:"ACER", botanical:"Acer rubrum", common:"Red Maple", category:"Tree", default_radius:3.0, default_spacing:20.0, palette:"STREET" },
    Plant { code:"PMON", botanical:"Pittosporum moniliferum", common:"Cape Pittosporum", category:"Tree", default_radius:2.5, default_spacing:16.0, palette:"STREET" },
    Plant { code:"RHIN", botanical:"Rhaphiolepis indica", common:"Indian Hawthorn", category:"Shrub", default_radius:1.5, default_spacing:4.0, palette:"STREET" },
    Plant { code:"MUCA", botanical:"Muhlenbergia capillaris", common:"Pink Muhly Grass", category:"Grass", default_radius:1.5, default_spacing:3.0, palette:"NATURALISTIC" },
    Plant { code:"LOMI", botanical:"Lomandra longifolia", common:"Longleaf Lomandra", category:"Grass", default_radius:1.5, default_spacing:3.0, palette:"NATIVE" },
    Plant { code:"HEAR", botanical:"Heteromeles arbutifolia", common:"Toyon", category:"Shrub", default_radius:2.0, default_spacing:7.0, palette:"NATIVE" },
];

#[derive(Clone, Copy)]
struct Palette {
    name: &'static str,
    description: &'static str,
    members: &'static [&'static str],
}

const NATIVE_MEMBERS: &[&str] = &["QAGR","ARUB","LOMI","HEAR"];
const STREET_MEMBERS: &[&str] = &["ACER","PMON","RHIN"];
const NATURAL_MEMBERS: &[&str] = &["MUCA","LOMI","HEAR"];

const PALETTES: &[Palette] = &[
    Palette { name:"NATIVE", description:"Starter native / habitat-oriented palette", members:NATIVE_MEMBERS },
    Palette { name:"STREET", description:"Starter streetscape palette", members:STREET_MEMBERS },
    Palette { name:"NATURALISTIC", description:"Starter naturalistic matrix palette", members:NATURAL_MEMBERS },
];

fn find_plant(code: &str) -> Option<Plant> {
    PLANTS.iter().copied().find(|p| p.code.eq_ignore_ascii_case(code))
}
fn find_palette(name: &str) -> Option<Palette> {
    PALETTES.iter().copied().find(|p| p.name.eq_ignore_ascii_case(name))
}
fn parse_arg(cmd: &str, index: usize) -> Option<String> {
    cmd.split_whitespace().nth(index + 1).map(|s| s.to_string())
}
fn parse_num(s: &str) -> Result<f64,String> {
    s.parse::<f64>().map_err(|_| format!("Expected number, got '{}'", s))
}

fn plant_xdata(p: Plant, size: &str, role: &str, area_id: Option<&str>) -> ExtendedDataRecord {
    let mut r = ExtendedDataRecord::new(XDATA_APP);
    r.add_value(XDataValue::String("schema=2".into()));
    r.add_value(XDataValue::String("object_type=plant".into()));
    r.add_value(XDataValue::String(format!("plant_code={}",p.code)));
    r.add_value(XDataValue::String(format!("botanical={}",p.botanical)));
    r.add_value(XDataValue::String(format!("common={}",p.common)));
    r.add_value(XDataValue::String(format!("category={}",p.category)));
    r.add_value(XDataValue::String(format!("size={}",size)));
    r.add_value(XDataValue::String(format!("role={}",role)));
    if let Some(id)=area_id {
        r.add_value(XDataValue::String(format!("area_id={}",id)));
    }
    r
}

fn area_xdata(id:&str,palette:&str,spacing:f64)->ExtendedDataRecord{
    let mut r=ExtendedDataRecord::new(XDATA_APP);
    r.add_value(XDataValue::String("schema=2".into()));
    r.add_value(XDataValue::String("object_type=planting_area".into()));
    r.add_value(XDataValue::String(format!("area_id={}",id)));
    r.add_value(XDataValue::String(format!("palette={}",palette)));
    r.add_value(XDataValue::String(format!("spacing={:.3}",spacing)));
    r
}

fn add_plant(host:&mut dyn HostApi,p:Plant,x:f64,y:f64,size:&str,role:&str,area_id:Option<&str>){
    let mut c=Circle::from_coords(x,y,0.0,p.default_radius);
    c.common.layer="L-PLNT-SYMB".into();
    c.common.extended_data.upsert_record(plant_xdata(p,size,role,area_id));
    host.add_entity(EntityType::Circle(c));
}

struct PlacePlant { p: Plant, size: &'static str }
impl InteractiveCommand for PlacePlant {
    fn prompt(&self)->String{
        format!("Place {} ({}) — specify insertion point",self.p.common,self.p.code)
    }
    fn on_point(&mut self,pt:[f64;3])->CommandStep{
        let mut c=Circle::from_coords(pt[0],pt[1],pt[2],self.p.default_radius);
        c.common.layer="L-PLNT-SYMB".into();
        c.common.extended_data.upsert_record(plant_xdata(self.p,self.size,"individual",None));
        CommandStep::CommitAndEnd(EntityType::Circle(c))
    }
}

struct PlaceArea {
    palette: Palette,
    spacing: f64,
    first: Option<[f64;3]>,
    id: String,
}
impl InteractiveCommand for PlaceArea {
    fn prompt(&self)->String{
        if self.first.is_none(){format!("Planting Area {} — specify first corner",self.id)}
        else{"Specify opposite corner".into()}
    }
    fn on_point(&mut self,pt:[f64;3])->CommandStep{
        if self.first.is_none(){self.first=Some(pt);return CommandStep::NeedPoint;}
        let a=self.first.unwrap(); let b=pt;
        let points=vec![
            Vector2::new(a[0],a[1]),
            Vector2::new(b[0],a[1]),
            Vector2::new(b[0],b[1]),
            Vector2::new(a[0],b[1]),
        ];
        let mut pl=LwPolyline::from_points(points);
        pl.common.layer="L-PLNT-AREA".into();
        pl.common.extended_data.upsert_record(area_xdata(&self.id,self.palette.name,self.spacing));
        CommandStep::CommitAndEnd(EntityType::LwPolyline(pl))
    }
}

fn grid_points(x1:f64,y1:f64,x2:f64,y2:f64,spacing:f64)->Vec<(f64,f64)>{
    let (minx,maxx)=(x1.min(x2),x1.max(x2));
    let (miny,maxy)=(y1.min(y2),y1.max(y2));
    let mut pts=Vec::new();
    if spacing<=0.0{return pts;}
    let mut y=miny+spacing/2.0;
    while y<maxy {
        let mut x=minx+spacing/2.0;
        while x<maxx {pts.push((x,y));x+=spacing;}
        y+=spacing;
    }
    pts
}

fn naturalized_points(x1:f64,y1:f64,x2:f64,y2:f64,spacing:f64)->Vec<(f64,f64)>{
    let (minx,maxx)=(x1.min(x2),x1.max(x2));
    let (miny,maxy)=(y1.min(y2),y1.max(y2));
    let mut pts=Vec::new();
    if spacing<=0.0{return pts;}
    let mut row=0usize;
    let mut y=miny+spacing*0.55;
    while y<maxy {
        let offset=if row%2==0{0.0}else{spacing*0.48};
        let mut col=0usize;
        let mut x=minx+spacing*0.55+offset;
        while x<maxx {
            let jx=(((row*37+col*17)%11) as f64-5.0)*spacing*0.025;
            let jy=(((row*19+col*29)%13) as f64-6.0)*spacing*0.02;
            pts.push((
                (x+jx).clamp(minx+spacing*0.2,maxx-spacing*0.2),
                (y+jy).clamp(miny+spacing*0.2,maxy-spacing*0.2)
            ));
            col+=1; x+=spacing;
        }
        row+=1; y+=spacing*0.88;
    }
    pts
}

fn report_catalog(host:&mut dyn HostApi){
    let mut out=String::from("LAND ARCH TOOLS v0.0.2 — PLANT CATALOG\n\n");
    out.push_str("CODE   BOTANICAL NAME                 COMMON NAME              CATEGORY  SPACING  PALETTE\n");
    out.push_str("-------------------------------------------------------------------------------------------\n");
    for p in PLANTS {
        out.push_str(&format!("{:<6} {:<30} {:<24} {:<8} {:>7.1}  {}\n",
            p.code,p.botanical,p.common,p.category,p.default_spacing,p.palette));
    }
    host.push_output(&out);
}
fn report_palettes(host:&mut dyn HostApi){
    let mut out=String::from("LAND ARCH TOOLS v0.0.2 — PLANT PALETTES\n\n");
    for pal in PALETTES {
        out.push_str(&format!("{}\n{}\n  {}\n\n",pal.name,pal.description,pal.members.join(", ")));
    }
    host.push_output(&out);
}
fn report_schedule(host:&mut dyn HostApi){
    let doc=host.document();
    let mut counts=BTreeMap::<String,usize>::new();
    let mut area_count=0usize;
    for entity in doc.entities(){
        if let Some(record)=entity.common().extended_data.records().iter().find(|r|r.application_name==XDATA_APP){
            let mut object_type="";
            let mut code=None::<String>;
            for value in &record.values {
                if let XDataValue::String(s)=value {
                    if let Some(v)=s.strip_prefix("object_type="){object_type=v;}
                    if let Some(v)=s.strip_prefix("plant_code="){code=Some(v.to_string());}
                }
            }
            if object_type=="planting_area"{area_count+=1;}
            if object_type=="plant"{
                if let Some(c)=code{*counts.entry(c).or_default()+=1;}
            }
        }
    }
    let mut out=String::from("LAND ARCH TOOLS v0.0.2 — PLANT SCHEDULE\n\n");
    out.push_str("CODE   BOTANICAL NAME                 COMMON NAME                 QTY\n");
    out.push_str("---------------------------------------------------------------------\n");
    for (code,qty) in counts{
        if let Some(p)=find_plant(&code){
            out.push_str(&format!("{:<6} {:<30} {:<27} {:>4}\n",p.code,p.botanical,p.common,qty));
        }
    }
    out.push_str(&format!("\nPlanting areas: {}\n",area_count));
    host.push_output(&out);
}

struct LandArchModule;
impl CadModule for LandArchModule {
    fn id(&self)->&'static str{"landarch"}
    fn title(&self)->&'static str{"Land Arch Tools"}
    fn ribbon_groups(&self)->Vec<RibbonGroup>{
        vec![
            RibbonGroup{title:"Project".into(),tools:vec![
                RibbonItem::LargeTool(ToolDef{id:"LA_HELP".into(),label:"Help".into(),icon:IconKind::Glyph("ⓘ"),event:ModuleEvent::Command("LA_HELP".into())}),
                RibbonItem::LargeTool(ToolDef{id:"LA_PALETTES".into(),label:"Palettes".into(),icon:IconKind::Glyph("☘"),event:ModuleEvent::Command("LA_PALETTES".into())}),
            ]},
            RibbonGroup{title:"Planting".into(),tools:vec![
                RibbonItem::LargeTool(ToolDef{id:"LA_TREE".into(),label:"Place Tree".into(),icon:IconKind::Glyph("T"),event:ModuleEvent::Command("LA_TREE".into())}),
                RibbonItem::LargeTool(ToolDef{id:"LA_SHRUB".into(),label:"Place Shrub".into(),icon:IconKind::Glyph("S"),event:ModuleEvent::Command("LA_SHRUB".into())}),
                RibbonItem::LargeTool(ToolDef{id:"LA_AREA".into(),label:"Planting Area".into(),icon:IconKind::Glyph("A"),event:ModuleEvent::Command("LA_AREA NATIVE 4".into())}),
                RibbonItem::LargeTool(ToolDef{id:"LA_PLANTS".into(),label:"Plant Catalog".into(),icon:IconKind::Glyph("P"),event:ModuleEvent::Command("LA_PLANTS".into())}),
            ]},
            RibbonGroup{title:"Populate".into(),tools:vec![
                RibbonItem::LargeTool(ToolDef{id:"LA_GRID".into(),label:"Grid Populate".into(),icon:IconKind::Glyph("G"),event:ModuleEvent::Command("LA_GRID QAGR 10 10 100 60 20".into())}),
                RibbonItem::LargeTool(ToolDef{id:"LA_NATURALIZE".into(),label:"Naturalize".into(),icon:IconKind::Glyph("N"),event:ModuleEvent::Command("LA_NATURALIZE MUCA 10 10 100 60 3".into())}),
            ]},
            RibbonGroup{title:"Documentation".into(),tools:vec![
                RibbonItem::LargeTool(ToolDef{id:"LA_LABEL".into(),label:"Label Plant".into(),icon:IconKind::Glyph("L"),event:ModuleEvent::Command("LA_LABEL QAGR".into())}),
                RibbonItem::LargeTool(ToolDef{id:"LA_SCHEDULE".into(),label:"Plant Schedule".into(),icon:IconKind::Glyph("#"),event:ModuleEvent::Command("LA_SCHEDULE".into())}),
            ]},
        ]
    }
}

struct LandArchPlugin;
impl BuiltinPlugin for LandArchPlugin {
    fn manifest(&self)->&'static PluginManifest{&MANIFEST}
    fn ribbon(&self)->Box<dyn CadModule>{Box::new(LandArchModule)}
    fn dispatch(&self,host:&mut dyn HostApi,cmd:&str)->bool{
        let command=cmd.split_whitespace().next().unwrap_or(cmd);
        match command {
            "LA_HELP"=>{
                host.push_info("Land Arch Tools v0.0.2 commands: LA_TREE [CODE], LA_SHRUB [CODE], LA_AREA [PALETTE] [SPACING], LA_GRID CODE X1 Y1 X2 Y2 SPACING, LA_NATURALIZE CODE X1 Y1 X2 Y2 SPACING, LA_PLANTS, LA_PALETTES, LA_LABEL CODE, LA_SCHEDULE");
                true
            }
            "LA_PLANTS"=>{report_catalog(host);true}
            "LA_PALETTES"=>{report_palettes(host);true}
            "LA_TREE"=>{
                let code=parse_arg(cmd,0).unwrap_or_else(||"QAGR".into());
                match find_plant(&code){
                    Some(p) if p.category=="Tree"=>{
                        host.push_undo("Land Arch Tools - Place Tree");
                        host.start_interactive(Box::new(PlacePlant{p,size:"24-inch box"}));
                    }
                    Some(p)=>host.push_error(&format!("{} is {}, not a tree.",p.code,p.category)),
                    None=>host.push_error(&format!("Unknown plant code: {}",code)),
                } true
            }
            "LA_SHRUB"=>{
                let code=parse_arg(cmd,0).unwrap_or_else(||"RHIN".into());
                match find_plant(&code){
                    Some(p) if p.category=="Shrub"||p.category=="Grass"=>{
                        host.push_undo("Land Arch Tools - Place Shrub/Grass");
                        host.start_interactive(Box::new(PlacePlant{p,size:"5-gallon"}));
                    }
                    Some(p)=>host.push_error(&format!("{} is {}, not a shrub/grass.",p.code,p.category)),
                    None=>host.push_error(&format!("Unknown plant code: {}",code)),
                } true
            }
            "LA_AREA"=>{
                let pname=parse_arg(cmd,0).unwrap_or_else(||"NATIVE".into());
                let spacing=parse_arg(cmd,1).and_then(|s|s.parse::<f64>().ok()).unwrap_or(4.0);
                match find_palette(&pname){
                    Some(pal)=>{
                        let id=format!("PA-{:04}",host.document().entities().count()+1);
                        host.push_undo("Land Arch Tools - Planting Area");
                        host.start_interactive(Box::new(PlaceArea{palette:pal,spacing,first:None,id}));
                    }
                    None=>host.push_error(&format!("Unknown palette: {}",pname)),
                } true
            }
            "LA_GRID"=>{
                let a:Vec<&str>=cmd.split_whitespace().skip(1).collect();
                if a.len()!=6{host.push_error("Usage: LA_GRID CODE X1 Y1 X2 Y2 SPACING");return true;}
                let p=match find_plant(a[0]){Some(v)=>v,None=>{host.push_error("Unknown plant code");return true;}};
                let nums:Result<Vec<f64>,String>=a[1..].iter().map(|s|parse_num(s)).collect();
                let n=match nums{Ok(v)=>v,Err(e)=>{host.push_error(&e);return true;}};
                if n[4]<=0.0{host.push_error("Spacing must be > 0.");return true;}
                let pts=grid_points(n[0],n[1],n[2],n[3],n[4]);
                for (x,y) in &pts{add_plant(host,p,*x,*y,"5-gallon","grid",None);}
                host.push_undo("Land Arch Tools - Grid Populate");host.set_dirty();
                host.push_info(&format!("Placed {} {} plants.",pts.len(),p.code)); true
            }
            "LA_NATURALIZE"=>{
                let a:Vec<&str>=cmd.split_whitespace().skip(1).collect();
                if a.len()!=6{host.push_error("Usage: LA_NATURALIZE CODE X1 Y1 X2 Y2 SPACING");return true;}
                let p=match find_plant(a[0]){Some(v)=>v,None=>{host.push_error("Unknown plant code");return true;}};
                let nums:Result<Vec<f64>,String>=a[1..].iter().map(|s|parse_num(s)).collect();
                let n=match nums{Ok(v)=>v,Err(e)=>{host.push_error(&e);return true;}};
                if n[4]<=0.0{host.push_error("Spacing must be > 0.");return true;}
                let pts=naturalized_points(n[0],n[1],n[2],n[3],n[4]);
                for (x,y) in &pts{add_plant(host,p,*x,*y,"5-gallon","naturalized",None);}
                host.push_undo("Land Arch Tools - Naturalize");host.set_dirty();
                host.push_info(&format!("Placed {} naturalized {} plants.",pts.len(),p.code)); true
            }
            "LA_LABEL"=>{
                let code=parse_arg(cmd,0).unwrap_or_else(||"QAGR".into());
                match find_plant(&code){
                    Some(p)=>{
                        struct LabelPlant{p:Plant}
                        impl InteractiveCommand for LabelPlant{
                            fn prompt(&self)->String{format!("Select a {} to place its label",self.p.common)}
                            fn needs_object_pick(&self)->bool{true}
                            fn on_object_pick(&mut self,_handle:Handle,pt:[f64;3])->CommandStep{
                                let mut t=Text::with_value(format!("{} — {}",self.p.code,self.p.common),Vector3::new(pt[0]+0.5,pt[1]+0.5,pt[2])).with_height(1.5);
                                t.horizontal_alignment=TextHorizontalAlignment::Left;
                                t.vertical_alignment=TextVerticalAlignment::Baseline;
                                t.common.layer="L-ANNO-PLNT".into();
                                CommandStep::CommitAndEnd(EntityType::Text(t))
                            }
                        }
                        host.push_undo("Land Arch Tools - Label Plant");
                        host.start_interactive(Box::new(LabelPlant{p}));
                    }
                    None=>host.push_error(&format!("Unknown plant code: {}",code)),
                } true
            }
            "LA_SCHEDULE"=>{report_schedule(host);true}
            _=>false,
        }
    }
}

ocs_plugin_api::export_plugin!(LandArchPlugin);
