# Creating Main

```bash
Starting at root ($) module.
> create Procedure at: main

args:
>> argc: Int64
>> argv: Vec<String>
>>

return-type:
>> Int64

path: (default: .)
>>

Created Procedure at .main

> list

$: Module
.main: Procedure
    .args: (argc: Int64, argv: Vec of String)
    .return-type: Int64
    .body: List of Expr 
    
> focus .main.body

> list

$.main.body: List of Expr

> insert

$.main.body.[0]:
>> print "Hello World!"
$.main.body.[1]:
>> return 0
>>

Inserted 2 expressions into $.main.body.
Type-checking consistent.

> list

$.main.body: [
    print "Hello World!"
    return 0
]

> focus ..

> list

$.main: Procedure
    .args: (argc: Int64, argv: Vec of String)
    .return-type: Int64
    .body: List of Expr
    
> show .

main: Procedure(argc: Int64, argv: Vec of String, => Int64) [
    print "Hello World!
    return 0
]

> freeze .

$.main is now immutable.

> thaw .

$.main is now mutable.

> focus ..

> list

$
    .main: Procedure(argc: Int64, argv: Vec of String, => Int64) [... 2]
        .args
        .return-type
        .body

> exit
```

```bash
Starting at root ($) module.

> list

$
    .main: Procedure(argc: Int64, argv: Vec of String, => Int64)
        .args
        .return-type
        .body

> 
``` 
