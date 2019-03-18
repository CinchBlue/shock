# The Shock Programming System

(This is just a mockup for now. To be expanded on later.)

## Introduction

The Shock programming system (Shock) is intended to be an all-in-one 
compiler/interpreter that is capable of normal execution of Shock programs, 
but also manipulation of the structure of programs themselves at "edit-time."

This enables extreme flexibility in analysis and development of programs, as 
you now have the power to query and mutate programs' information and structure
on-the-fly, but also to create new "edit-time" procedures (macros) to 
programmatically manipulate and analyze other programs as any other data 
structure.

In short, Shock attempts to combine (when editing programs):

- blurring of compile-time vs. run-time
- metaprogramming w/ homoiconicity
- an online compilation + program editing model with static typing

in order to achieve:

- flexibility in metaprogramming to achieve complex programming tasks
- performant incremental compilation and program editing
- a more efficient overall workflow for software development

## Design

Shock is centered around the concept of a ***programming server (progserv)*** --
a server that serves as the single-point-of-contact between the programmer and 
program. In short, the progserv exposes an API that is used to create, read, 
update, and delete parts of the program.

### Data Representation in Shock

Programs in Shock are modeled as directed, multi-dimensional graphs (where 
nodes are of one unique type but edges may consist of multiple distinct types
that thus produce multiple different adjacency matrices, each representing a
dimension of the graph).

Each "node" in the graph is called a **place**. Places are intended to be 
locations of direct **editor objects** such as integers, scopes, 
procedures, or references to locations of other nodes. In short, places 
represents places where data can be put.

Each "edge" in the graph is called a **relation**. Relations describe the 
relationships between POs.

Two special kinds of relations are **possessions** and **references**. 
Posessions imply ownership of one place over another, while references do not.

It is intended that the possession itself forms a semi-lattice over the set 
of POs in a program graph. Thus, each place has a unique owner, except for 
the **root place**, which is allowed to stand alone, owning itself.

In summary, every PO has:

- a place
- an owner
- a list of possessions
- a list of references
- a list of other relations
  
### Primitive Types in Shock

Shock is intended to support the following primitive types:

```
RelationId
PlaceId
Bool
Byte
Int
Float
Character
String
Path
Name (valid variable name)
```

### Compound Types in Shock

Shock is also intended to support the following compound types:

```
Either(T...)
Pair(T, U)
Tuple(T...)
Array(T, Int)
Binding(S for T)
Struct(Binding(Name for T) ...)
Enum(Name ...)
Variant(T ...)
```

### Custom Types in Shock

Shock is also intended to support custom types:

```
Alias(String for T)
Procedure(Struct(...) to T) 
```

### Reflective Types in Shock

Shock is intended to support the following reflective types:

```
Place = Struct {
    id: PlaceId
    data: T
    owner: RelationId
    possessions: Tuple(Binding of: RelationId ...)
    references: Tuple(Binding of: RelationId ...)
    relations: Tuple(Binding of: RelationId ...)
}

Relation = Struct {
    id: RelationId
    type: String
    from: PlaceId
    to: PlaceId
}

Procedure = Struct {
    place: Place
    args: Struct(Binding of: T ...)
    return-type: String
    body: Tuple(RelationId ...)
}

Binding = Pair(Name, T)
```

### Conversion between inline places and indirect relations

- Relations act as proxies for actual places.

- Places can normally just be referred to by other places.

- Places are also just proxies for direct data.

## Edit-time vs. Run-time

Shock has two primary forms of computation and "function abstraction": 
**macros** and **procedures**.

- Macros can be invoked only at edit-time.
- Procedures can be invoked at both edit-time and run-time.

Procedures are not allowed to take in or produce reflective types as objects.
Instead, they must use references or possessions.

Macros can be used to perform actions such as:

- Connect to a database to be used in the editor
- Get input for the editor from the user
- Perform a query over the attributes of another macro or procedure

Procedures are often used to do actions such as:

- Perform arithmetic
- Get input at run-time
- Perform an HTTP request in a compiled program.

One thing that makes macros special is that they are offered special 
edit-time powers:

- Procedural Reflection
- Editor Reflection
- Metaprogramming Behaviors

### Procedural Reflection

- Procedures are themselves structures within Shock. Macros give the freedom 
to query, write, and read expressions and procedures within the editor 
environment.

- Using procedural reflection, you can answer questions like:
  - How many times do I call `print` within a procedure?
  - Is this macro eligible to be converted into a procedure?
  - What would happen if I optimize this procedure using peephole optimizations?
 
### Editor Reflection

As Shock itself is simply another environment in which to perform IO and 
computation, with macros, you can also define behaviors that hook into the 
editor environment.

For example, Shock may invoke a certain type of "startup behavior" when it 
starts up and then it hooks into a default prompt. By using editor 
reflection, it can be possible to have Shock automatically run a edit-time 
macro of your choice to start, say, a graphical environment, or to pull 
changes from other people in the future.

You should also be able to define your own editor commands, as well as create
your own editor plugins and invoke them, package them up. In fact, you may 
be able to extend the Shock environment to compile its own procedures down 
to machine code!

Using editor reflection, you can answer questions like:

- How might a summary of my editing history look or be aggregated?
- Can I start up another process automatically when I start Shock?
- Can I re-define the behavior of some "basic commands" like `create`?

### Metaprogramming Behaviors

One of Shock's stronger intended behaviors is the ability to produce programs
using programs. The idea is that since procedures and expressions themselves
are simply just commands which are themselves first-class edit-time data
structures, you can produce those data structures and create procedures.

Shock's semi-homoiconic nature also makes it amenable to both read and write 
programs that produce Shock programs.
