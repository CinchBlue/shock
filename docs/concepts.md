start -> workspace

# Terms

- place = a location that a user is associated with at any given time.
- start-place = the place where you begin your session and do work
- workspace = a set of unique places.
- package = a set of unique places

## The Graph

- code-graph = everywhere that you can access within the system
- current-place = where you currently are in the codebase

- you start in a place
- you can move into other places
- workspaces are comprised of a root place and a set of other reachable sub places that never loop into themselves
- packages are sort of like "mini workspaces" that may contain each other I guess?

## The Possession Model

- places possess other places.
- only the root-place owns itself
- "owning" implies "contains"
- owners are unique

## The Reference Model

- places can reference other places
- "reference" implies "depends on"
- references can be many.
- references can also be broken

### Freezing

- places can be made mutable or immutable
- "freeze" means to make immutable, and "unfreezes" implies to make mutable.
- freezing is non-invasive to the actual place -- it only changes the user's view of the place

## Baking

- places that fit a setup can be "baked" (compiled). This means that a certain set of states of places are essentially associated with a certain "baked" artifact and then are fit together.
- you can reference baked object, but can't "unbake" them.
- essentially, baked objects are libraries that you don't have source code to

## Primitive Data

- place
    - A place is a "node" in a "code graph"
    - A place has a unique identifier that it is associated with.
    - Places can have references to other places by containing their ID
    - primitive properties:
        - {id, name, attribute-tuple}
    - places can have attributes as well as connected places.
- tuple
    - a places that is directly associated with other places.
    - primitive properties:
        - {id, ordered-set-of-place-ids}  
- byte
    - has a value from 0 to 255
- c-string
    - an ordered set of bytes
- boolean
    - 0 or 1
- attribute
    - a pair of a (string, primitive-data)
- attribute-table
    - a set of attribute-pairs

## Basic operations

- select-place
- unselect-place
- insert-place
- change-focus
- focus-undo
- focus-redo
- delete-place
- modify-attribute

## The Data Structure

- We want a flat data structure. To do this, we consider all places as their own nodes in the graph
- Child nodes are referred to by Locally Unique ID.

```
procedure fib: Int n -> Int;
    if n less than or equal to 1: return 1;
    else: return n * fib(n-1);
```

// BECOMES

```
fib: {
    type: Procedure
    args: [n: Int]
    return-type: Int
    body: [
        $fib.if
        $fib.else
    ]
}

fib.if: {
    test-expression: $fib.if.test-expression
    body: {
        $fib.if.return
    }
}

fib.if.test-expression: {
    expression: $fib.if.test-expression.lte
}

fib.if.test-expression.lte {
    type: Procedure-Application
    procedure: @primitive.less-than-or-equal-to(Int, Int, => Bool) 
    args: [
        $fib.if.test-expression.lte.lhs,
        $fib.if.test-expression.lte.rhs,
    ]
}

fib.if.test-expression.lte.lhs {
    type: Variable-Reference
    value: $fib.args.n
}

fib.if.test-expression.lte.rhs {
    type: Int 
    value: 1
}

fib.if.return {
    type: Continuation-Application
    cont: @.std.control-flow.return
    value: $fib.if.return.value
}

fib.if.return.value {
    type: Int
    value: 0
}

fib.else: {
    body: {
        $fib.else.return
    }
}

fib.else.return: {
    type: Continuation-Application
    cont: @.std.control-flow.return
    value: $fib.else.return.expr
}

fib.else.return.expr: {
    type: Procedure-Application
    procedure: @primitive.multiply(Int, Int, => Int)
    args: [
        $fib.else.return.expr.lhs,
        $fib.else.return.expr.rhs
    ]
}

fib.else.return.expr.lhs: {
    type: Variable-Reference
    value: @fib.args.n
}

fib.else.return.expr.rhs: {
    type: Procedure-Application
    procedure: @fib
    args: [
        @fib.args.n
    ]
}
```


Boolean (Bool)
Byte (Byte)
Signed Integer (Signed-Int, Int)
Unsigned Integer (Unsigned-Int, UInt)

Floating-Point (Float)

Character (Char)
C-String (String)

Expression (Expr)
Place (Place)

- A place has:
    - id
    - name
    - typelist
    - attributes
- A relation has:
    - id
    - name
    - typelist
    - attributes:


```
# No semantic indentation
define Procedure fact(Int n => Int):
    if (n <= 1):
        return 1
    else:
        return n * fact(n-1)

# # is for comments
# @ is for reference
# $ is for ownership
# . is used to separate place names, and to prefix variables
# ~ is used to represent the local root
# / is used to represent the global root
# : is used to denote blocks and to specify argument names
# () is used to denote argument lists
# [] is used to denote primitive lists
# {} is used to denote structs/maps
```


    % shock
    Welcome to Shock 1.0.0.
    No workspace defined for current directory.
    > Choose directory: ./
    Okay. Continuing.
    > procedure main(argc: Int, argv: String[])





# Grammar

shock-shell or `sh2` has a basic grammar:

```
hspace
    ::= [ \t]+
lowercase-alpha
    ::= [a-z]
id-chars
    ::= [A-Za-z0-9-_]
identifier
    ::= <lowercase-alpha> <id-chars>*
newline
    ::= <CRLF>|<LF>
end-command
    ::= <newline>|";"
argument-name
    ::= <identifier>
expression-core
    ::= <identifier> (<hspace> <expression>)? <hspace> (<argument-name>: <hspace> <expression>)*
expression
    ::= "(" <expression-core> ")"
command
    ::= <expression-core> <end-command>
```


