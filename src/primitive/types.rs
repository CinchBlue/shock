extern crate uuid;

use uuid::Uuid;
use std::collections::HashMap;
use std::fmt;

/// Represents the different types of "stand-alone" primitive data.
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveData {
    Bool(bool),
    Byte(u8),
    Int(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Name(String),
}

/// A Place represents a primitive "object" in Shock's primitive system, and can
/// have attributes. Many Places can form an arbitrary graph.
///
/// More data structures can be schema-encoded in primitive + attribute structure.
///
/// NOTE: primitive is ownership-agnostic -- choose your own memory management.
pub struct Place {
    id: PlaceId,
    attr: HashMap<String, AttributeData>,
}

/// All places on a given Shock server have a unique id number.
pub type PlaceId = uuid::Uuid;

/// Attributes on Places can either be another Place, or primitive data.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeData {
    Place(PlaceId),
    Data(PrimitiveData),
}

impl Clone for Place {
    fn clone(&self) -> Self {
        Place { id: Place::generate_id(), attr: self.attr.clone()}
    }
}

impl fmt::Debug for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "primitive {{ id: {}, attr: {:?} }}", self.id.to_hyphenated(), self.attr)
    }
}

impl Place {
    /// Generates a unique id for a primitive.
    ///
    /// NOTE: Not sure if this should be unique across just the primitive type or all objects.
    pub fn generate_id() -> PlaceId {
        Uuid::new_v4()
    }
    
    /// Constructs a new primitive.
    pub fn new(id: PlaceId, attr: HashMap<String, AttributeData>) -> Self {
        Place{id, attr}
    }
    
    /// Checks whether if the primitive contains the key as an attribute.
    pub fn contains_key(&self, key: &String) -> bool {
        self.attr.contains_key(key)
    }
    
    /// Immutable get from a primitive's attribute map by key.
    pub fn get_attr(&self, key: &String) -> Option<&AttributeData> {
        self.attr.get(key)
    }
    
    /// Idempotent put into a primitive's attribute map by key.
    pub fn put_attr(&mut self, key: String, value: AttributeData) {
        self.attr.insert(key, value);
    }
    
    /// Get the key's value from the primitive, and create a new primitive from it.
    pub fn reify_attr(&mut self, key: &String, id: PlaceId) -> Option<Place> {
        match self.attr.get(key) {
            // If no key, do not create it with a value key.
            None => None,
            // If key, create the primitive with a single attribute named "value".
            Some(attr_data) => {
                let mut attribute_map: HashMap<String, AttributeData> = HashMap::new();
                attribute_map.insert(String::from("value"), attr_data.clone());
                Some(Place { id, attr: attribute_map})
            },
        }
    }
    
    /// Deletes an attribute if it is there; no-op of it's not.
    pub fn remove_attr(&mut self, key: &String) {
        self.attr.remove(key);
    }
    
    /// Immutable get for the id
    pub fn get_id(&self) -> PlaceId {
        self.id
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn set_and_get_attributes() {
        // Given: a Place
        let mut p = Place{ id: Place::generate_id(), attr: HashMap::new() };
        
        // When: we put some data
        p.put_attr("value".to_string(), AttributeData::Data(PrimitiveData::Int(1)));
        
        // Then: we should get the same thing out
        assert_eq!(
            Some(&AttributeData::Data(PrimitiveData::Int(1))),
            p.get_attr(&"value".to_string()));
        
        // Then: if we try to get something that's not in the, we should get None
        assert_eq!(
            None,
            p.get_attr(&"meow".to_string()));
    }
    
    #[test]
    fn check_if_contains_key() {
        // Given: a Place
        let mut p = Place{ id: Place::generate_id(), attr: HashMap::new() };
        println!("{:?}", p);
    
        // When: we put some data
        p.put_attr("value".to_string(), AttributeData::Data(PrimitiveData::Int(1)));
   
        // Then: we should be able to check if the Place contains an attribute name
        assert_eq!(true, p.contains_key(&"value".to_string()));
        assert_eq!(false, p.contains_key(&"should not be in attr map".to_string()));
    }
    
    #[test]
    fn reify_attribute_as_place() {
        let attribute_data =  AttributeData::Data(PrimitiveData::Int(1));
        let attribute_key = "meow".to_string();
       
        // Given: a Place that has some AttributeData in it
        let mut p = Place{ id: Place::generate_id(), attr: HashMap::new() };
        p.put_attr(attribute_key.clone(), attribute_data.clone());
        
        // When: you reify the attribute as a Place
        let id = Place::generate_id();
        let reified = p.reify_attr(&attribute_key, id);
        
        // Then: the reified attribute data should be in `value`
        assert_eq!(Some(&attribute_data), reified.unwrap().get_attr(&"value".to_string()));
        
        // When: you reify an attribute that's not on the Place's attribute map
        let non_reified = p.reify_attr(&"should not exist".to_string(), Place::generate_id());
        
        // Then: you should get None
        assert_eq!(true, non_reified.is_none());
    }
    
    #[test]
    fn remove_attribute() {
        let attribute_data =  AttributeData::Data(PrimitiveData::Int(1));
        let attribute_key = "meow".to_string();
        
        // Given: a Place that has some AttributeData in it
        let mut p = Place{ id: Place::generate_id(), attr: HashMap::new() };
        p.put_attr(attribute_key.clone(), attribute_data.clone());
        
        // When: you try and remove the attribute
        p.remove_attr(&attribute_key);
        
        // Then: you should get None if you try to get it again.
        assert_eq!(None, p.get_attr(&attribute_key));
    }
    
}
