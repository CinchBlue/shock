# Shock Python Mock

This is a mock/prototype for the design of Shock in Python.

## Basic Features

- Features a basic programming language with gradual typing
  that can be compiled to native and also be used as a shell
  and scripting language.
- Allows command-based actions on both the code and
  the editor itself to perform program editing, analysis, and
  integration.
  
## The Language

```
For all intents and purposes, semicolons are treated the same
as newlines in expressions.

# Literals
42                                      # Integer
3.1415                                  # Float
"string"                                # String (UTF-8)
'c'                                     # Character
$hello                                  # Symbol
hello                                   # Variable/Label
Hello                                   # Type
_                                       # BoundValue

# Composite
<Identifier>: <Expr>                    # LabeledExpr
{ <Expr> (; <Expr>)* }                  # Block
[<Expr> (, <Expr>)*]                    # List (or array)
( <LabeledExpr> (, <LabeledExpr>)* )    # Struct (labeled tuple)


# Expressions
<Expr>      ::= <Variable> <Argument>*
              | <Literal>
              | <Composite>
              | <Expr> => <Expr>
              | <Comment>

<Argument>  ::= <LabeledExpr>
              | <BoundValue>
              | <Expr>
              | [ <Expr> ]

<Comment>   ::= <LineComment>
              | <MultiLineComment>
              | <BlockComment>
              | <ListComment>
              | <StructComment>

@ this is a comment
@@ This is a multi-line comment
@[ this is a block comment ]
@{
    this is an unstructured list comment
}@
@(
    name: "structComment",
    type: StructComment, 
)@
```

