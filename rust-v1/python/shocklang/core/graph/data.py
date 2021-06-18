import uuid
from typing import Dict, Tuple, Sequence, List


class Place:
    """Representation of a node"""

    def __init__(self, **kwargs):
        self.id = uuid.uuid4()
        self.attr = {}
        for key, value in kwargs.items():
            self.attr[key] = value

    def __str__(self) -> str:
        place_type = self.attr['primtype'] \
            if 'primtype' in self.attr \
            else '?'
        place_value = '"{}"'.format(self.attr['name']) \
            if 'name' in self.attr \
            else (self.attr['value'] if 'value' in self.attr else '?')
        return '({}): {}'.format(place_type, place_value, self.id)

    def __repr__(self) -> str:
        return "P[{}] {}".format(
            self.id,
            self.attr)


class Link:
    """Representation of an edge"""

    def __init__(
            self,
            src_place_id: uuid.UUID,
            dest_place_id: uuid.UUID,
            name: str,
            **kwargs):
        self.id = uuid.uuid4()
        self.src_place_id = src_place_id
        self.dest_place_id = dest_place_id
        self.name = name
        self.attr = {}
        for key, value in kwargs.items():
            self.attr[key] = value

    def __str__(self):
        return "L[{}] ({}) {} => {} {}".format(
            self.id,
            self.name,
            self.src_place_id,
            self.dest_place_id,
            self.attr)

    def __repr__(self):
        return "({}) {} -> {}".format(
            self.name,
            self.src_place_id,
            self.dest_place_id)


class GraphStore:
    """Store in which to store places and links"""

    def __init__(self):
        self.places = {}
        self.links = {}
        self.links_by_from = {}
        self.links_by_to = {}

    def insert_place(self, place: Place):
        self.places[place.id] = place

    def get_place(self, place_id: uuid.UUID) -> Place:
        return self.places.get(place_id, None)

    def remove_place(self, place_id: uuid.UUID):
        if place_id in self.places:
            del self.places[place_id]

    def insert_link(self, link: Link):
        if (link.src_place_id in self.places.keys() and
                link.dest_place_id in self.places.keys()):
            # Insert in the link_by_id dict
            self.links[link.id] = link

            # Insert link_by_id into dict
            if not (link.src_place_id in self.links_by_from):
                self.links_by_from[link.src_place_id] = {}
            self.links_by_from[link.src_place_id][link.id] = link

            if not (link.dest_place_id in self.links_by_to):
                self.links_by_to[link.dest_place_id] = {}
            self.links_by_to[link.dest_place_id][link.id] = link

    def get_link(self, link_id: uuid.UUID) -> Link:
        return self.links.get(link_id, None)

    def get_links_from(self, place_id: uuid.UUID) -> Dict[uuid.UUID, Link]:
        return self.links_by_from.get(place_id, {})

    def get_links_to(self, place_id: uuid.UUID) -> Dict[uuid.UUID, Link]:
        return self.links_by_to.get(place_id, {})

    def remove_link(self, link_id: uuid.UUID):
        if link_id in self.links:
            link = self.links[link_id]
            del self.links_by_from[link.src_place_id][link_id]
            del self.links_by_to[link.dest_place_id][link_id]
            del self.links[link_id]
