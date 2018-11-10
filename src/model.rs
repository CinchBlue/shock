extern crate uuid;

use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum PrimitiveData {
    PBool(bool),
    PByte(u8),
    PInt(i64),
    PFloat(f64),
    PString(String),
}

#[derive(Debug, Clone)]
pub enum PlaceData {
    Place(Place),
    Data(PrimitiveData),
}

pub type PlaceId = uuid::Uuid;

#[derive(Debug)]
pub struct Place {
    id: PlaceId,
    name: std::string::String,
    attr: HashMap<String, PlaceData>,
}

impl Clone for Place {
    fn clone(&self) -> Self {
        Place { id: Place::generate_id(), name: self.name.clone(), attr: self.attr.clone()}
    }
}

impl Place {
    /// Generates a unique id for a Place.
    ///
    /// NOTE: Not sure if this should be unique across just the Place type or all objects.
    pub fn generate_id() -> PlaceId {
       Uuid::new_v4()
    }
   
    /// Constructs a new Place.
    pub fn new(name: String, attr: HashMap<String, PlaceData>) -> Self {
        Place{id: Place::generate_id(), name, attr}
    }
   
    /// Checks whether if the Place contains the key as an attribute.
    pub fn contains_key(&self, key: String) -> bool {
        self.attr.contains_key(&key)
    }
   
    /// Immutable get from a Place's attribute map by key.
    pub fn get_attr(&self, key: String) -> Option<&PlaceData> {
        self.attr.get(&key)
    }
   
    /// Idempotent put into a Place's attribute map by key.
    pub fn put_attr(&mut self, key: String, value: PlaceData) {
        self.attr.insert(key, value);
    }
    
    /// Get the key's value from the Place, and create a new Place from it.
    pub fn reify_attr(&mut self, name: String, key: String) -> Place {
        match self.attr.get(key.as_str()) {
            // If no key, do not create it with a value key.
            None => Place {id: Place::generate_id(), name, attr: HashMap::new() },
            // If key, create the place with a single attribute named "value".
            Some(place_data) => {
                let mut attribute_map: HashMap<String, PlaceData> = HashMap::new();
                attribute_map.insert(String::from("value"), place_data.clone());
                Place { id: Place::generate_id(), name, attr: attribute_map}
            },
        }
    }
}