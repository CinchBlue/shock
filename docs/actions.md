# Insert (in)
```
insert item: <Expression> at: <Path>
```
**Insert** an expression into the path. The expression is evaluated first at
command-time, and then inserted after the position of the cursor.


# Remove (rm)
```
remove at: <Path>
```
**Remove** a place at a path.


# Freeze (fr)
```
freeze at: <Path>
```
**Freeze** the place at the path.

Freezing makes the place immutable to the current context. This means that the
current context will not be able to modify the frozen item (e.g. you will not
be able to remove a frozen place).

Freezing is idempotent.


# Thaw (th)
```
thaw at: <Path>
```
**Thaw** the place at the path. This makes a place unfrozen. 

Thawing is idempotent.


# If (if)
```
if cond: <Boolean> then: <Expression> else: <Expression>
```
Evaluates the *then* expression **If** the *cond* expression evaluates to True.


 
# Let (let)
```
let name: <String> be: <Expression>
```
Evaluate the *be* expression and **Let** the *name* be bound to that in the
 current command-time scope.
 
 
# Create (cre)
```
create type: <PlaceType> as: <String> at: <Path>
```
**Create** a *type* in place *at* the given path.


# Construct (cons)
```
construct type: <Type> with: <Argument-List>
```
**Constructs** a literal Type *with* the arguments in place.

