import sys

try:
    import curses as curses
except ImportError:
    import windows_curses as curses

from shocklang.core.data import Place, Link, GraphStore
from shocklang.core.eval import BasicEvaluator
from shocklang.graph import GraphTraversalContext

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


def command_show(depth: str = '5'):
    """Prints the current expression to depth [0] (default: 3)"""
    cxt.show(int(depth))


commands['s'] = command_show


def command_stack():
    """Prints the current stack of places in the current context"""
    for i, s in enumerate(list(map(
            lambda x: repr(gs.get_place(x)),
            cxt.place_stack))):
        print('[{:3d}]: {}'.format(i, s))


commands['stack'] = command_stack

print('Welcome to Shock! An example expression has already been initialized '
      'for you. To exit, press Ctrl+C (Ctrl+Z for Windows) twice.')

commands['exit'] = lambda: sys.exit(0)


def repl():
    while True:
        raw_command = input('> ')
        command = raw_command.split()
        if command and len(command) > 0 and command[0] in commands:
            try:
                result = commands[command[0]](*command[1:])
                if result:
                    print(result)
            except SystemExit as err:
                break
            except Exception as err:
                print(repr(err))
        else:
            print('"{}" is not a valid command (see "?")'.format(raw_command))


repl()
