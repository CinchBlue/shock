import uuid
from typing import Dict, List

from shocklang.core.data import GraphStore, Link
from shocklang.core.eval import BasicEvaluator


class GraphTraversalContext:
    """Represents a traversal context on the graph."""

    def __init__(self, start_place: uuid.UUID, gs: GraphStore):
        self.place = gs.get_place(start_place)
        self.place_stack = [start_place]
        self.gs = gs

    def outl(self, place_id: uuid.UUID = None) -> Dict[uuid.UUID, Link]:
        """Gets links going out from the current place by link id"""
        return self.gs.get_links_from(place_id if place_id else self.place.id)

    def inl(self, place_id: uuid.UUID = None) -> Dict[uuid.UUID, Link]:
        """Gets links going in to the current place by link id"""
        return self.gs.get_links_to(place_id if place_id else self.place.id)

    def outnl(self, place_id: uuid.UUID = None) -> Dict[str, Link]:
        """Gets links going out from the current place by attribute name"""
        out_links = self.outl(place_id)
        return {v.name: v for k, v in out_links.items()}

    def innl(self, place_id: uuid.UUID = None) -> Dict[str, Link]:
        """Gets links going in to the current place by attribute name"""
        in_links = self.inl(place_id)
        return {v.name: v for k, v in in_links.items()}

    def focus(self, *args: List[str]):
        """Focus a connected connected place by attrbiute name"""
        for attr_name in args:
            self.focus_name(attr_name)

    def focus_name(self, attr_name: str):
        """Traverse from the current place to another by way of link name"""
        attr_links = self.outnl()
        if attr_name in attr_links:
            self.place = self.gs.get_place(attr_links[attr_name].dest_place_id)
            self.place_stack.append(self.place.id)

    def focus_link(self, id: uuid.UUID):
        """Traverse from the current place to another though out link id"""
        out_links = self.outl()
        if id in out_links:
            self.place = self.gs.get_place(out_links[id].dest_place_id)
            self.place_stack.append(self.place.id)

    def focus_place(self, id: uuid.UUID):
        self.place = self.gs.get_place(id)
        self.place_stack.append(self.place.id)

    def prev(self):
        """Show the previous place in the place stack"""
        if self.place_stack and len(self.place_stack) > 2:
            print(self.gs.get_place(self.place_stack[-2]))
        else:
            self.curr()

    def back(self):
        """If possible, pops the place stack by 1"""
        if self.place_stack and len(self.place_stack) > 1:
            self.place_stack.pop()
            self.place = self.gs.get_place(self.place_stack[-1])
        self.curr()

    def curr(self):
        """Prints out a representation of the current place"""
        print(self.place)

    def eval(self):
        """Evaluates the current expression starting from the current place"""
        evaluator = BasicEvaluator()
        print(evaluator.evaluate(self.place, self.gs))

    def set_attr(self, name: str, item):
        self.place.attr[name] = item

    def get_attr(self, name: str):
        return self.place.attr[name]

    def show_level(
            self,
            link_name: str,
            place_id: uuid.UUID,
            curr_level: int = 0,
            max_level: int = 0):
        place = self.gs.get_place(place_id)
        if not place: return
        print('|{}'.format('-' * curr_level), end=' ')
        place_str = str(place)
        if curr_level > max_level:
            print('...')
            return
        print('{} => {}'.format(link_name, place_str))
        for out_name, out_link in self.outnl(place_id).items():
            if place:
                self.show_level(
                    out_name, out_link.dest_place_id, curr_level + 1, max_level)

    def show(self, level: int = 1):
        """Gives a summary of the current place + out links"""
        print(repr(self.place))
        self.curr()
        for out_name, out_link in self.outnl().items():
            self.show_level(out_name, out_link.dest_place_id, 1, level)
            # place = self.gs.get_place(out_link.dest_place_id)
            # if place:
            #     place_str = str(place)
            #     print('|{}'.format('--'*level), end=' ')
            #     print('{} => {}'.format(out_name, place_str))
            #     if level > 0:
            #         self.show()
