use crate::primitive::types::Place;
use crate::primitive::types::PlaceId;
use std::collections::HashMap;
use crate::placemodel::pathtypes::Path;
use crate::primitive::types::AttributeData;
use crate::placemodel::pathtypes::Link;

pub trait PlaceStore {
    fn put_place(&mut self, place: Place);
    fn get_place(&self, id: &PlaceId) -> Option<&Place>;
    fn delete_place(&mut self, id: &PlaceId);
    
    
    fn put_linked_place(&mut self, from: &PlaceId, attr: String, mut place: Place) -> Option<Link> {
        if let Some(from_place) = self.get_place(from) {
            let mut modified_from_place = from_place.clone();
            modified_from_place.put_attr(attr.clone(), AttributeData::Place(place.get_id()));
            let result = Some(Link::new(from_place.get_id(), place.get_id(), attr).unwrap());
            self.put_place(modified_from_place);
            self.put_place(place);
            result
        } else {
            None
        }
    }
    
    fn verify_path(&self, root: &PlaceId, path: &Path) -> bool {
        // Start traversal from the root.
        let mut expected_place_id= root;
        // Verify that the first item in the path is indeed the root.
        if let Some((first_place_id, first_attr_name)) = path.get_traversal_list().first() {
            if first_place_id != expected_place_id {
                return false;
            }
        }
        // Verify each link in the path.
        let mut iter = path.get_traversal_list().iter().skip(1);
        while let Some((curr_place_id, attr_name)) = iter.next() {
            // If the next place is not in the store, fail.
            if let Some(curr_place) = self.get_place(expected_place_id) {
                // If the attribute doesn't exist, fail.
                if let Some(attr_data) = curr_place.get_attr(attr_name) {
                    // If the attribute is of the wrong type, fail.
                    if let AttributeData::Place(next_place_id) = attr_data {
                        // Otherwise, we have found the next place in the correct attribute spot on the current place.
                        // Continue (if we are at the last one, this is an unnecessary write).
                        expected_place_id = next_place_id;
                        // If the current place is not the next expected place in the chain, fail.
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct HashMapPlaceStore {
    store: HashMap<PlaceId, Place>
}

impl HashMapPlaceStore {
    pub fn new() -> HashMapPlaceStore {
        HashMapPlaceStore { store: HashMap::new() }
    }
}

impl PlaceStore for HashMapPlaceStore {
    
    fn put_place(&mut self, place: Place) {
        self.store.insert(place.get_id(), place);
    }
    
    fn get_place(&self, id: &PlaceId) -> Option<&Place> {
        self.store.get(id)
    }
    
    fn delete_place(&mut self, id: &PlaceId) {
        self.store.remove(id);
    }
}

#[cfg(test)]
mod tests {
    use crate::primitive::types::{Place, AttributeData};
    use std::collections::HashMap;
    use crate::placemodel::storage::{HashMapPlaceStore, PlaceStore};
    use crate::placemodel::pathtypes::{Link, Path};
   
    #[test]
    fn verify_valid_path() {
        // Given: a PlaceStore
        let mut store: HashMapPlaceStore = HashMapPlaceStore::new();
       
        // Given: places
        let mut place1 = Place::generate_new();
        println!("place1: {:#?}", place1);
        let mut place2 = Place::generate_new();
        println!("place2: {:#?}", place2);
        let mut place3 = Place::generate_new();
        println!("place3: {:#?}", place3);
  
        // Given: we put places into the store with links between them
        let mut valid_path = Path::with_root(place1.get_id());
        store.put_place(place1.clone());
        println!("state: {:#?}", store);
        valid_path.push_link(store.put_linked_place(&place1.get_id(), "foo".to_string(), place2.clone()).unwrap());
        println!("state: {:#?}", store);
        valid_path.push_link(store.put_linked_place(&place2.get_id(), "bar".to_string(), place3.clone()).unwrap());
        println!("state: {:#?}", store);
       
        
        // When: we try to verify a valid path through the attribute chain
        // Then: verification should succeed
        assert_eq!(true, store.verify_path(&place1.get_id(), &valid_path));
        
        
        
        // Given: an invalid path
        let mut invalid_path = Path::with_root(place3.get_id());
        invalid_path.push_link(Link::new(place3.get_id(), place1.get_id(), "meow".to_string()).unwrap());
        
        // When: we try to verify an invalid path through the attribute chain
        // Then: verification should fail
        assert_eq!(false, store.verify_path(&place3.get_id(), &invalid_path));
    }
}