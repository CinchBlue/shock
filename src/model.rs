extern crate uuid;

use uuid::Uuid;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveData {
    Bool(bool),
    Byte(u8),
    Int(i64),
    Float(f64),
    String(String),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum PlaceData {
    Place(PlaceId),
    Data(PrimitiveData),
}

pub type PlaceId = uuid::Uuid;

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

impl fmt::Debug for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "place {{ id: {}, name: {}, attr: {:?} }}", self.id.to_hyphenated(), self.name, self.attr)
    }
}


impl Place {
    /// Generates a unique id for a place.
    ///
    /// NOTE: Not sure if this should be unique across just the place type or all objects.
    pub fn generate_id() -> PlaceId {
        Uuid::new_v4()
    }
    
    /// Constructs a new place.
    pub fn new(name: String, attr: HashMap<String, PlaceData>) -> Self {
        Place{id: Place::generate_id(), name, attr}
    }
    
    /// Checks whether if the place contains the key as an attribute.
    pub fn contains_key(&self, key: String) -> bool {
        self.attr.contains_key(&key)
    }
    
    /// Immutable get from a place's attribute map by key.
    pub fn get_attr(&self, key: String) -> Option<&PlaceData> {
        self.attr.get(&key)
    }
    
    /// Idempotent put into a place's attribute map by key.
    pub fn put_attr(&mut self, key: String, value: PlaceData) {
        self.attr.insert(key, value);
    }
    
    /// Get the key's value from the place, and create a new place from it.
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

#[derive(Clone)]
pub struct Path {
    parent: PlaceId,
    current: PlaceId,
    name_list: Vec<String>,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path {{ parent: {}, current: {}, name_list: {:?} }}",
               self.parent.to_hyphenated(),
               self.current.to_hyphenated(),
               self.name_list)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path_string = self.name_list.join(".");
        write!(f, "{}", path_string)
    }
}
