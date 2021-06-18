use crate::primitive::types::PlaceId;

/// A Link is a representation of a relationship between two Places through an attribute.
///
/// The Link can be invalidated with respect to a certain place graph if:
/// 1) The places are no longer valid for traversal (e.g. one of the two places was deleted from the place store)
/// 2) The attribute relationship "chain" is broken (e.g. the attribute name between the two nodes has changed)
#[derive(Debug, Clone)]
pub struct Link {
    prev: PlaceId,
    curr: PlaceId,
    attr_name: String,
}

impl Link {
    pub fn new(prev: PlaceId, curr: PlaceId, attr_name: String) -> Option<Link> {
        if attr_name.is_empty() {
            return None;
        }
        Some(Link{ prev, curr, attr_name })
    }
    
    pub fn get_prev(&self) -> &PlaceId { &self.prev }
    pub fn get_curr(&self) -> &PlaceId { &self.curr }
    pub fn get_attr_name(&self) -> &String { &self.attr_name }
}

/// A Path is a representation of many Links, forming a possibly-valid traversal between two Places that may be
/// separated by more than one attribute relationship.
///
/// Once again, a Path can be invalidated with the same conditions as the PathLink
pub struct Path {
    traversal_list: Vec<(PlaceId, String)>,
}

impl Path {
    pub fn with_root(place: PlaceId) -> Path {
        Path { traversal_list: vec![(place, "".to_string())] }
    }
    
    pub fn from_single_link(link: Link) -> Path {
        Path { traversal_list: vec![(link.prev, "".to_string()), (link.curr, link.attr_name)] }
    }
    
    pub fn get_traversal_list(&self) -> &Vec<(PlaceId, String)> {
        &self.traversal_list
    }
    
    pub fn push_link(&mut self, link: Link) -> bool {
        if let Some(last_item) = self.traversal_list.last_mut() {
            if link.prev == last_item.0 {
                if link.attr_name.is_empty() {
                    return false;
                }
                self.traversal_list.push((link.curr, link.attr_name));
                return true;
            }
        }
        false
    }
    
    pub fn pop_link(&mut self, link: Link) -> bool {
        if self.traversal_list.len() > 0 {
            return self.traversal_list.pop() != None;
        }
        false
    }
}
