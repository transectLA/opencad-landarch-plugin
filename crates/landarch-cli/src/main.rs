fn main(){
 let args:Vec<String>=std::env::args().collect();let cmd=args.get(1).map(String::as_str).unwrap_or("help");
 match cmd{
  "plants"=>print!("{}",landarch_core::format_plant_catalog()),"palettes"=>print!("{}",landarch_core::format_palette_catalog()),
  "materials"=>print!("{}",landarch_core::format_material_catalog()),"furnishings"=>print!("{}",landarch_core::format_furnishing_catalog()),"amenities"=>print!("{}",landarch_core::format_amenity_catalog()),
  _=>println!("landarch-cli: plants | palettes | materials | furnishings | amenities"),
 }
}
