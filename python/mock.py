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
        return '{}: {}'.format(place_type, place_value)

    def __repr__(self) -> str:
        return "P[{}] {}".format(
            self.id,
            self.attr)


class Link:
    """Representation of an edge"""

    def __init__(
            self,
            from_place: uuid.UUID,
            to_place: uuid.UUID,
            name: str,
            **kwargs):
        self.id = uuid.uuid4()
        self.from_place = from_place
        self.to_place = to_place
        self.name = name
        self.attr = {}
        for key, value in kwargs.items():
            self.attr[key] = value

    def __str__(self):
        return "L[{}] ({}) {} => {} => {} {}".format(
            self.id,
            self.name,
            self.from_place,
            self.to_place,
            self.attr)

    def __repr__(self):
        return "({}) {} -> {}".format(
            self.name,
            self.from_place,
            self.to_place)


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
        if (link.from_place in self.places.keys() and
                link.to_place in self.places.keys()):
            # Insert in the link_by_id dict
            self.links[link.id] = link

            # Insert link_by_id into dict
            if not (link.from_place in self.links_by_from):
                self.links_by_from[link.from_place] = {}
            self.links_by_from[link.from_place][link.id] = link

            if not (link.to_place in self.links_by_to):
                self.links_by_to[link.to_place] = {}
            self.links_by_to[link.to_place][link.id] = link

    def get_link(self, link_id: uuid.UUID) -> Link:
        return self.links.get(link_id, None)

    def get_links_from(self, place_id: uuid.UUID) -> Dict[uuid.UUID, Link]:
        return self.links_by_from.get(place_id, {})

    def get_links_to(self, place_id: uuid.UUID) -> Dict[uuid.UUID, Link]:
        return self.links_by_to.get(place_id, {})

    def remove_link(self, link_id: uuid.UUID):
        if link_id in self.links:
            link = self.links[link_id]
            del self.links_by_from[link.from_place][link_id]
            del self.links_by_to[link.to_place][link_id]
            del self.links[link_id]


class BasicEvaluator:
    """Evaluates basic arithmetic expressions in place graph form"""

    def __init__(self):
        pass

    def evaluate(self, root: Place, graph: GraphStore) -> int:
        # print("Place: {}".format(root))
        final_value = 0
        if root.attr["primtype"] == "Procedure":
            operands = graph.get_links_from(root.id)
            for link in operands.values():
                # print("Link: {}".format(link))
                if link.name.startswith("op"):
                    op_result = self.evaluate(
                        graph.get_place(link.to_place),
                        graph)
                    if root.attr["name"] == "+":
                        final_value += op_result
            return final_value
        elif root.attr["primtype"] == "Integer":
            return root.attr["value"]
        else:
            raise RuntimeError("What the duck this wasn't a Proc or Int!")


class GraphTraversalContext:
    """Represents a traversal context on the graph."""
    def __init__(self, start_place: uuid.UUID, gs: GraphStore):
        self.place = gs.get_place(start_place)
        self.place_stack = [start_place]
        self.gs = gs

    def outl(self) -> Dict[uuid.UUID, Link]:
        """Gets links going out from the current place by link id"""
        return gs.get_links_from(self.place.id)

    def inl(self) -> Dict[uuid.UUID, Link]:
        """Gets links going in to the current place by link id"""
        return gs.get_links_to(self.place.id)

    def outnl(self) -> Dict[str, Link]:
        """Gets links going out from the current place by attribute name"""
        out_links = self.outl()
        return {v.name: v for k, v in out_links.items()}

    def innl(self) -> Dict[str, Link]:
        """Gets links going in to the current place by attribute name"""
        in_links = self.inl()
        return {v.name: v for k, v in in_links.items()}

    def focus(self, attr_name: str):
        """Focus a connected connected place by attrbiute name"""
        self.focus_name(attr_name)

    def focus_name(self, attr_name: str):
        """Traverse from the current place to another by way of link name"""
        attr_links = self.outnl()
        if attr_name in attr_links:
            self.place = gs.get_place(attr_links[attr_name].to_place)
            self.place_stack.append(self.place.id)

    def focus_link(self, id: uuid.UUID):
        """Traverse from the current place to another though out link id"""
        out_links = self.outl()
        if id in out_links:
            self.place = gs.get_place(out_links[id].to_place)
            self.place_stack.append(self.place.id)

    def focus_place(self, id: uuid.UUID):
        self.place = self.gs.get_place(id)
        self.place_stack.append(self.place.id)

    def prev(self):
        """Show the previous place in the place stack"""
        if self.place_stack and len(self.place_stack) > 2:
            print(gs.get_place(self.place_stack[-2]))
        else:
            self.curr()

    def back(self):
        """If possible, pops the place stack by 1"""
        if self.place_stack and len(self.place_stack) > 1:
            self.place_stack.pop()
            self.place = gs.get_place(self.place_stack[-1])
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

    def show(self) -> str:
        """Gives a summary of the current place + out links"""
        self.curr()
        for out_name, out_link in self.outnl().items():
            place = self.gs.get_place(out_link.to_place)
            if place:
                place_str = str(place)
                print('|-- {} => {}'.format(out_name, place_str))


gs = GraphStore()

# Create an arithmetic expression going top-down for (+ 0 .. 11)
toplevel_root = Place(primtype="Procedure", name="+")
root = toplevel_root
gs.insert_place(root)
for i in range(0, 10):
    # Create the left-hand operand
    lhs = Place(primtype="Integer", value=i)
    gs.insert_place(lhs)
    left_link = Link(root.id, lhs.id, "op1", primtype="Possession")
    gs.insert_link(left_link)

    # Create the right-hand operand
    rhs = Place(primtype="Procedure", name="+")
    gs.insert_place(rhs)
    right_link = Link(root.id, rhs.id, "op2", primtype="Possession")
    gs.insert_link(right_link)

    # Move onto the rhs as root
    root = rhs

# Create the left-hand operand
final_lhs = Place(primtype="Integer", value=10)
gs.insert_place(final_lhs)
left_link = Link(root.id, final_lhs.id, "op1", type="Possession")
gs.insert_link(left_link)

# Create the right-hand operand
final_rhs = Place(primtype="Integer", value=11)
gs.insert_place(final_rhs)
right_link = Link(root.id, final_rhs.id, "op2", type="Possession")
gs.insert_link(right_link)

assert BasicEvaluator().evaluate(toplevel_root, gs) == 66

# Now that we have a "graph," let's traverse over it.

# Here, we are setting up the interactive GraphTraversalContext
cxt = GraphTraversalContext(toplevel_root.id, gs)

# We put in some easy commands
commands = {}
commands['c'] = cxt.show
commands['f'] = cxt.focus
commands['b'] = cxt.back
commands['e'] = cxt.eval
commands['i'] = cxt.innl
commands['o'] = cxt.outnl

def command_help():
    """Prints every current bound command and its documentation string"""
    for command_name, command in commands.items():
        print('{}: {}'.format(command_name, command.__doc__))


commands['?'] = command_help

print('Welcome to Shock! An example expression has already been initialized '
      'for you. To exit, press Ctrl+C (Ctrl+Z for Windows) twice.')
while True:
    raw_command = input('> ')
    command = raw_command.split()
    if command[0] in commands:
        try:
            result = commands[command[0]](*command[1:])
            if result:
                print(result)
        except Exception as err:
            print(repr(err))
    else:
        print('"{}" is not a valid command (see "?")'.format(raw_command))
