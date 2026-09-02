#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PlantCategory{Tree,Shrub,Grass,Groundcover,Perennial}
impl PlantCategory{pub const fn as_str(self)->&'static str{match self{Self::Tree=>"Tree",Self::Shrub=>"Shrub",Self::Grass=>"Grass",Self::Groundcover=>"Groundcover",Self::Perennial=>"Perennial"}}}

#[derive(Clone,Copy,Debug)]
pub struct Plant{pub code:&'static str,pub botanical:&'static str,pub common:&'static str,pub category:PlantCategory,pub default_spacing:f64,pub symbol_radius:f64,pub layer:&'static str,pub palette:&'static str}
#[derive(Clone,Copy,Debug)]
pub struct Palette{pub name:&'static str,pub description:&'static str,pub members:&'static [&'static str]}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Unit{SquareFeet,LinearFeet,Each}
impl Unit{pub const fn as_str(self)->&'static str{match self{Self::SquareFeet=>"SF",Self::LinearFeet=>"LF",Self::Each=>"EA"}}}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum MaterialCategory{Paving,Surfacing,Edge,Wall,Deck}
impl MaterialCategory{pub const fn as_str(self)->&'static str{match self{Self::Paving=>"Paving",Self::Surfacing=>"Surfacing",Self::Edge=>"Edge",Self::Wall=>"Wall",Self::Deck=>"Deck"}}}
#[derive(Clone,Copy,Debug)]
pub struct Material{pub code:&'static str,pub name:&'static str,pub category:MaterialCategory,pub unit:Unit,pub layer:&'static str,pub spec_section:&'static str,pub detail:&'static str,pub unit_cost:f64}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SiteCategory{Furnishing,Amenity}
impl SiteCategory{pub const fn as_str(self)->&'static str{match self{Self::Furnishing=>"Furnishing",Self::Amenity=>"Amenity"}}}
#[derive(Clone,Copy,Debug)]
pub struct SiteObject{pub code:&'static str,pub name:&'static str,pub category:SiteCategory,pub manufacturer:&'static str,pub model:&'static str,pub layer:&'static str,pub spec_section:&'static str,pub detail:&'static str,pub symbol_radius:f64,pub unit_cost:f64}
