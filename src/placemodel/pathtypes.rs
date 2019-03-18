use crate::primitive::types::PlaceId;

/// A Link is a representation of a relationship between two Places through an attribute.
///
/// The Link can be invalidated with respect to a certain place graph if:
/// 1) The places are no longer valid for traversal (e.g. one of the two places was deleted from the place store)
/// 2) The attribute relationship "chain" is broken (e.g. the attribute name between the two nodes has changed)
#[derive(Debug, Clone)]
pub struct Link {
    pub prev: PlaceId,
    pub curr: PlaceId,
    pub attr_name: String,
}

/// A Path is a representation of many Links, forming a possibly-valid traversal between two Places that may be
/// separated by more than one attribute relationship.
///
/// Once again, a Path can be invalidated with the same conditions as the PathLink
pub struct Path {
    place_list: Vec<PlaceId>,
    name_list: Vec<String>,
}

impl Path {
    pub fn from_single_link(link: PathLink) -> Path {
        Path { place_list: vec![link.prev, link.curr], name_list: vec![link.attr_name] }
    }
}
