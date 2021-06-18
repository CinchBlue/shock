# Creating Main 

```
> create Procedure as: main
```

Invokes the `create` command with (0: Procedure, as: main).

This will usually invoke some sort of interactive wizard that saves the
current continuation, and creates a new context within the environment.

```
get-bindings-list for args: (Name: Type)* 
> argc: Int
> argv: (Array String 5)
>
```

This is the context to collect arguments to a list of bindings.

Each line must consist of a variable binding. The context can 
incrementally check to see if each line is a variable binding.

When the user puts in an empty line, the context ends, and it
returns a list of bindings to the outer `create Procedure` context.

Now, `create Procedure` wants to get the arguments for the body, so it make 
invoke a context called `get-expressions` or such to get the body

```
body: Expression+ 
> print "Hello world!"
> return 0
> 

```

Here, the `get-expressions` context is used to get 1 or more expressions
that will get placed into the body of the `Procedure`. We can check each line
incrementally to see if it is valid at the time of creation as well.

```
Created main: Procedure
    /args: {argc: Int, argv: [Array of: String size: Int]}
    /body: (
        print "Hello world!"
        return 0
    )
```

Finally, we have created `main` as a `Procedure` that has attributes `.args` 
and `.body`. 




