# PrimitiveData

**PrimitiveData** can be:

- a String
- an Integer
- a Character
- a Byte
- a Boolean
- a Reference
- a Possession
- a Place


# Place

```
Place (
    id: PlaceId
    name: String
    attr: [(key: String, value: PrimitiveData)]  
)
```

**Place** is the fundamental unit of code in Shock. It represents an 
arbitrary node in the program. 

- They have a unique ID across the entire command environment.
- They have a non-unique name that is used to construct Paths.
- They contain a list of attributes as Strings as well as associated data.


# Path
```
Path = String or Vec<String>
```

A **Path** is a series of substrings that represents a path through the 
attribute tree/graph by keys.

There are special paths:

- `~` represents the root place of the current module.
- `$` represents the root place of the command environment.


# References/Possession

A **Possession** is an object to represent the unique ownership of one Place 
over another.

A **Reference** is an object to represent a non-unique reference of one Place
 to another.
 
- Possessive paths are marked with the prefix: `@`
- Referencing paths are marked with the prefix: `&`


# Container Types

- Homogenous List: `[]`
- Homogenous Map: `()`
- Hetergenous Tuple: `{}`
- Hetergenous Enum: `|`


 
 

