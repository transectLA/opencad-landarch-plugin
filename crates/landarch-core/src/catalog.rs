use crate::model::*;

pub static PLANTS:&[Plant]=&[
 Plant{code:"QAGR",botanical:"Quercus agrifolia",common:"Coast Live Oak",category:PlantCategory::Tree,default_spacing:25.0,symbol_radius:4.0,layer:"L-PLNT-TREE",palette:"NATIVE"},
 Plant{code:"ARUB",botanical:"Arbutus unedo",common:"Strawberry Tree",category:PlantCategory::Tree,default_spacing:18.0,symbol_radius:3.0,layer:"L-PLNT-TREE",palette:"NATIVE"},
 Plant{code:"ACER",botanical:"Acer rubrum",common:"Red Maple",category:PlantCategory::Tree,default_spacing:20.0,symbol_radius:3.0,layer:"L-PLNT-TREE",palette:"STREET"},
 Plant{code:"RHIN",botanical:"Rhaphiolepis indica",common:"Indian Hawthorn",category:PlantCategory::Shrub,default_spacing:4.0,symbol_radius:1.5,layer:"L-PLNT-SHRB",palette:"STREET"},
 Plant{code:"MUCA",botanical:"Muhlenbergia capillaris",common:"Pink Muhly Grass",category:PlantCategory::Grass,default_spacing:3.0,symbol_radius:1.5,layer:"L-PLNT-GRAS",palette:"NATURALISTIC"},
 Plant{code:"LOMI",botanical:"Lomandra longifolia",common:"Longleaf Lomandra",category:PlantCategory::Grass,default_spacing:3.0,symbol_radius:1.5,layer:"L-PLNT-GRAS",palette:"NATIVE"},
 Plant{code:"HEAR",botanical:"Heteromeles arbutifolia",common:"Toyon",category:PlantCategory::Shrub,default_spacing:7.0,symbol_radius:2.0,layer:"L-PLNT-SHRB",palette:"NATIVE"},
];
static NATIVE:&[&str]=&["QAGR","ARUB","LOMI","HEAR"];
static STREET:&[&str]=&["ACER","RHIN"];
static NATURAL:&[&str]=&["MUCA","LOMI","HEAR"];
pub static PALETTES:&[Palette]=&[
 Palette{name:"NATIVE",description:"Native / habitat-oriented starter palette",members:NATIVE},
 Palette{name:"STREET",description:"Streetscape starter palette",members:STREET},
 Palette{name:"NATURALISTIC",description:"Naturalistic matrix starter palette",members:NATURAL},
];

pub static MATERIALS:&[Material]=&[
 Material{code:"PAV-CONC",name:"Cast-in-place concrete paving",category:MaterialCategory::Paving,unit:Unit::SquareFeet,layer:"L-HARD-PAVE",spec_section:"32 13 13",detail:"1/L5.01",unit_cost:16.50},
 Material{code:"PAV-UP",name:"Concrete unit paving",category:MaterialCategory::Paving,unit:Unit::SquareFeet,layer:"L-HARD-PAVE",spec_section:"32 14 13",detail:"2/L5.01",unit_cost:24.00},
 Material{code:"PAV-DG",name:"Stabilized decomposed granite",category:MaterialCategory::Surfacing,unit:Unit::SquareFeet,layer:"L-HARD-SURF",spec_section:"32 15 40",detail:"3/L5.01",unit_cost:9.50},
 Material{code:"DECK-WOOD",name:"Exterior wood decking",category:MaterialCategory::Deck,unit:Unit::SquareFeet,layer:"L-HARD-DECK",spec_section:"06 15 00",detail:"4/L5.01",unit_cost:32.00},
 Material{code:"EDGE-STEEL",name:"Steel landscape edging",category:MaterialCategory::Edge,unit:Unit::LinearFeet,layer:"L-HARD-EDGE",spec_section:"32 94 00",detail:"5/L5.01",unit_cost:12.00},
 Material{code:"CURB-CIP",name:"Cast-in-place concrete curb",category:MaterialCategory::Edge,unit:Unit::LinearFeet,layer:"L-HARD-CURB",spec_section:"32 13 13",detail:"6/L5.01",unit_cost:48.00},
 Material{code:"WALL-CIP",name:"Cast-in-place concrete site wall",category:MaterialCategory::Wall,unit:Unit::LinearFeet,layer:"L-HARD-WALL",spec_section:"03 30 00",detail:"7/L5.01",unit_cost:425.00},
];

pub static FURNISHINGS:&[SiteObject]=&[
 SiteObject{code:"BENCH-01",name:"Backed site bench",category:SiteCategory::Furnishing,manufacturer:"Basis of design",model:"6-ft bench",layer:"L-SITE-FURN",spec_section:"12 93 00",detail:"1/L5.02",symbol_radius:3.0,unit_cost:2400.0},
 SiteObject{code:"TABLE-01",name:"Picnic table",category:SiteCategory::Furnishing,manufacturer:"Basis of design",model:"Accessible table",layer:"L-SITE-FURN",spec_section:"12 93 00",detail:"2/L5.02",symbol_radius:3.5,unit_cost:3200.0},
 SiteObject{code:"BIKE-01",name:"Bicycle rack",category:SiteCategory::Furnishing,manufacturer:"Basis of design",model:"Inverted-U",layer:"L-SITE-FURN",spec_section:"12 93 00",detail:"3/L5.02",symbol_radius:1.5,unit_cost:650.0},
 SiteObject{code:"TRASH-01",name:"Waste / recycling receptacle",category:SiteCategory::Furnishing,manufacturer:"Basis of design",model:"Dual stream",layer:"L-SITE-FURN",spec_section:"12 93 00",detail:"4/L5.02",symbol_radius:1.5,unit_cost:1800.0},
 SiteObject{code:"BOLL-01",name:"Site bollard",category:SiteCategory::Furnishing,manufacturer:"Basis of design",model:"Fixed bollard",layer:"L-SITE-FURN",spec_section:"32 17 43",detail:"5/L5.02",symbol_radius:0.75,unit_cost:950.0},
];

pub static AMENITIES:&[SiteObject]=&[
 SiteObject{code:"DRINK-01",name:"Drinking fountain / bottle filler",category:SiteCategory::Amenity,manufacturer:"Basis of design",model:"Outdoor accessible",layer:"L-SITE-AMEN",spec_section:"22 47 13",detail:"1/L5.03",symbol_radius:1.5,unit_cost:6200.0},
 SiteObject{code:"GRILL-01",name:"Park grill",category:SiteCategory::Amenity,manufacturer:"Basis of design",model:"Fixed charcoal grill",layer:"L-SITE-AMEN",spec_section:"11 68 00",detail:"2/L5.03",symbol_radius:1.75,unit_cost:1700.0},
 SiteObject{code:"FIRE-01",name:"Site fire feature",category:SiteCategory::Amenity,manufacturer:"Basis of design",model:"Outdoor fire feature",layer:"L-SITE-AMEN",spec_section:"11 68 00",detail:"3/L5.03",symbol_radius:3.0,unit_cost:8500.0},
 SiteObject{code:"DOG-01",name:"Dog waste station",category:SiteCategory::Amenity,manufacturer:"Basis of design",model:"Bag dispenser + bin",layer:"L-SITE-AMEN",spec_section:"12 93 00",detail:"4/L5.03",symbol_radius:1.0,unit_cost:900.0},
 SiteObject{code:"PLAY-01",name:"Play equipment zone",category:SiteCategory::Amenity,manufacturer:"Basis of design",model:"Play element",layer:"L-SITE-AMEN",spec_section:"11 68 13",detail:"5/L5.03",symbol_radius:5.0,unit_cost:25000.0},
 SiteObject{code:"SHADE-01",name:"Prefabricated shade structure",category:SiteCategory::Amenity,manufacturer:"Basis of design",model:"Shade canopy",layer:"L-SITE-AMEN",spec_section:"13 34 23",detail:"6/L5.03",symbol_radius:6.0,unit_cost:18000.0},
];

pub fn plant(code:&str)->Option<&'static Plant>{PLANTS.iter().find(|p|p.code.eq_ignore_ascii_case(code))}
pub fn palette(name:&str)->Option<&'static Palette>{PALETTES.iter().find(|p|p.name.eq_ignore_ascii_case(name))}
pub fn material(code:&str)->Option<&'static Material>{MATERIALS.iter().find(|p|p.code.eq_ignore_ascii_case(code))}
pub fn furnishing(code:&str)->Option<&'static SiteObject>{FURNISHINGS.iter().find(|p|p.code.eq_ignore_ascii_case(code))}
pub fn amenity(code:&str)->Option<&'static SiteObject>{AMENITIES.iter().find(|p|p.code.eq_ignore_ascii_case(code))}

pub fn format_plant_catalog()->String{let mut s=String::from("CODE      BOTANICAL NAME                     COMMON NAME               TYPE       SPACING\n------------------------------------------------------------------------------------\n");for p in PLANTS{s.push_str(&format!("{:<9} {:<34} {:<25} {:<10} {:>7.1}\n",p.code,p.botanical,p.common,p.category.as_str(),p.default_spacing));}s}
pub fn format_palette_catalog()->String{let mut s=String::new();for p in PALETTES{s.push_str(&format!("{} — {}\n  {}\n",p.name,p.description,p.members.join(", ")));}s}
pub fn format_material_catalog()->String{let mut s=String::from("CODE          MATERIAL                            UNIT  SPEC       DETAIL      UNIT COST\n-------------------------------------------------------------------------------------\n");for p in MATERIALS{s.push_str(&format!("{:<13} {:<35} {:<5} {:<10} {:<11} ${:>8.2}\n",p.code,p.name,p.unit.as_str(),p.spec_section,p.detail,p.unit_cost));}s}
pub fn format_furnishing_catalog()->String{format_objects(FURNISHINGS)}
pub fn format_amenity_catalog()->String{format_objects(AMENITIES)}
fn format_objects(items:&[SiteObject])->String{let mut s=String::from("CODE          ITEM                                SPEC       DETAIL      UNIT COST\n----------------------------------------------------------------------------\n");for p in items{s.push_str(&format!("{:<13} {:<35} {:<10} {:<11} ${:>8.2}\n",p.code,p.name,p.spec_section,p.detail,p.unit_cost));}s}
