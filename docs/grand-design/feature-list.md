# Feature List

## Basic Run-time Procedures

- let
- `+`
- `-`
- `*`
- `/`
- print 
- prompt
- convert obj: Expr to type: Type
- chain procs: (Proc)

## Basic Edit-time Macros

- place new: Type at: Path
- delete at: Path
- copy from: Path to: Path
- cons new: Type with: (Expr)
- move from: Path to: Path
- call proc: Proc args: (Expr)
- eval macro: Macro args: (Expr)
- alias from: Path to: Path

## Behaviors + Types Functionality

- Behaviors:
  - `Construct` tells you how to create a type given a set of arguments at 
  run-time. The type of `Construct`'s `cons (T)` is `([Binding from: 
  Name to: Expr] -> T)`
  - `Create` tells you how to create a type given a set of arguments at 
  edit-time. The type of `Construct`'s `create (T)` is `([Binding of: Expr] ->
   T)`
  - `Clone` tells you how to create a copy of an object. The type of Copy's 
  `clone (T)` is `(&T -> @T)`.
  - `Display` tells you how to print an object.

- Behaviors should be able to be specified on types.
- New types should be able to be specified.

