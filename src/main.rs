extern crate shock;

use shock::model::Place;
use std::collections::HashMap;

fn main() {
    println!("Hello world!");
    
    let p = Place::new("root".to_string(), HashMap::new());
    
    println!("{:?}", p);
}