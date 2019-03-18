use crate::primitive::types::Place;
use crate::primitive::types::PlaceId;
use std::collections::HashMap;

trait PlaceStore {
    fn put_place(&mut self, place: Place);
    fn get_place(&self, id: &PlaceId) -> Option<&Place>;
    fn delete_place(&mut self, id: &PlaceId);
}

struct HashMapPlaceStore {
    store: HashMap<PlaceId, Place>
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