extern crate shock;

use shock::model::{Place, PrimitiveData, PlaceData};
use std::collections::HashMap;

fn main() {
    println!("Hello world!");
    
    let mut p = Place::new("root".to_string(), HashMap::new());
    
    println!("{:?}", p);
    
    p.put_attr("type".to_string(), PlaceData::Data(PrimitiveData::PString("Place".to_string())));
    
    println!("{:?}", p);
}