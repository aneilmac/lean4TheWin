~~~admonish warning
- [x] Student understands `def` keyword in relation to declaration.
- [ ] Student understands basic type-specifier syntax (e.g `: Nat`.)
- [x] Student understands writing simple functions.
- [ ] Student understands they can call `[1, 2].reverse`, and
- [ ] Student understands lean file is executed in order.
- [ ]  `id` function
~~~

# Fun functions

## The `def` keword

It is often useful to _name_ an expression, such that we can refer to it later. For example:

~~~admonish example title=""
```lean
def three := 3

#eval three
```
3
~~~

`def` stands for _definition_. Here we say the definition of "three" 
is `3 : Nat`.

The type of `three` is implicitly guessed by Lean based on the expression on the right-hand side of `:=`. If want to explicitly give a
type to our definition, we add `: Type` _before_ the assignment 
operator, `:=`, like so:

~~~admonish example title=""
```lean
def five : Int := 5

#check five
```
five : Int
~~~

`five` can now obe used anywhere an `Int` is allowed.

~~~admonish example title=""
```lean
def five : Int := 5

def myList : List Int := [five, 2, -five, 4, five]

#eval myList
```
[5, 2, -5, 4, 5]
~~~

## Functions

`def` can also be used to define _functions_. You can add arguments like so:

~~~admonish example title=""
```lean

def double x := x * 2

#eval double 5
```
10
~~~

Here `x` is a an argument of double. We call it just like we called
function in the previous chapter, and may use it wherever an expression
of the given type is allowed.

~~~admonish example title=""
```lean
#eval [double 0, double 1, double 2, double 3, double 4]
```
[0, 2, 4, 6, 8]
~~~

~~~admonish example title=""
```lean
#eval double (double 6)
```
24
~~~

If we `#check` the type of `double` we see that Lean guessed `x` to 
be a `Nat`, and the return type to also be `Nat`:

~~~admonish example title=""
```lean
#check double
```
double (x : Nat) : Nat
~~~

We can use our usual `( : Type)` syntax to tell Lean than `x` should
be a different type.

~~~admonish example title=""
```lean

def double (x : Int) := x * 2

#check double
```
double (x : Int) : Int
~~~

Given that `x` is an `Int`, Lean guesses the type of the entire
expression to be `Int`.

Again, we can specific all the arguments _and_ the return type if 
we so wish. Here `double` takes a `Nat`, but return an `Int`.

~~~admonish example title=""
```lean
def double (x : Nat) : Int := x * 2

#check double
```
double (x : Nat) : Int
~~~

## Multiple arguments

Functions can take any number of arguments. 

~~~admonish example title=""
```lean
def add (x : Int) (y : Int) := x + y

#eval add 3 4
```
7
~~~

This can be quite wordy to type out! Lean does help here. Y can group
all argument of the same type together with one `(: Type)`.

~~~admonish example title=""
```lean
def addThree (x y z : Int) := x + y + z

#eval addThree 3 4 7
```
14
~~~

## Exploring function types with `#print`

We've seen `#eval` and `#check` to query Lean about an expression.

There is an additional command in our toolbox now: `#print`. When used with definitions, _print_ will print to the infoview the entire definition.


~~~admonish example title=""
```lean
def double x := x * 2
#print double
```
def double : Nat → Nat := fun x ↦ x * 2
~~~

Yikes! This _kinda_ looks like our function we wrote called `double`. 
But it looks like it has been mangled. Why does it look like that?

Let's compare the definitions of `double` we know of so far:

```lean
-- Minimal definition, with types guessed by Lean.
def double x := x * 2

-- Definition with explicit return type
def double x : Nat := x * 2

-- Definition with explicit parameter type and return value.
def double (x : Nat) : Nat := x * 2

-- Definition given to us by the `#print` function.
def double : Nat → Nat := fun x ↦ x * 2
```

All these functions are _equivalent_. They are different ways of 
writing the same thing. The final definition is the "true" definition
used by Lean under the hood. Everything else is _syntactical sugar_, 
nicer ways of writing `double` which are more human readable.

First off, let's look at the type of `double`. The type is:
`Nat → Nat`.

First of we are introduced to a new symbol : `→`, 
sometimes know as the function arrow. 
(written as `\rightarrow`,`\->`, or just plain `->`). 
We read `Nat → Nat` as a type that _takes_ a `Nat` and produces a `Nat`.

And if we think about it, this makes sense! `double` has type `Nat → Nat`, while
`double 5` has type `Nat`!

~~~admonish info

Another example is our definition of `add`:

```lean
def add (x y : Int) := x + y
```

The type of `add` is actually `Int -> Int -> Int`! 
It is a function that takes two  `Int`s, and produces an `Int`.
~~~

Next, let's look at the definition to the right of `:=`

```lean
fun x ↦ x * 2
```

We are introduced to some new syntax again.
This is what's known as a _lambda_ function.

lambda functions appear in different languages under different names:
anonymous functions, closures, callbacks. But the idea is the same, it is a
function expression without a name.

A lambda function starts with the word `fun` followed by some named arguments.
Then anything to the right of `↦` (also known as `\mapsto` or `=>`) is the 
function definition.

Here we only have 1 argument `x`, and the definition is `x * 2`!

Let's try and define `add_three_and_double` in terms of a lambda function.

~~~admonish example title=""
```lean
def add_three_and_double := fun x y z => (x + y + z) * 2
#check add_three_and_double
```
add_three_and_double (x y z : Nat) : Nat
~~~

Here, we've left out the type signature on `add_three_and_double` to let
Lean guess it. Note that `#check` has prettified the type to make it look
like:

```lean
def add_three_and_double (x y z : Nat) := (x + y + z) * 2
```

### Partial function application and currying

The first question you might be asking yourself is: what use is knowing any of
this? I understand `add` has a type signature of `Nat -> Nat -> Nat`? But what
practical use is that to me?

Having a strong understanding of function arrows is important for theorem
proving. It also helps explain to use a surprising behavior of Lean!

~~~admonish example title=""
```lean
def add (x y : Nat) := x + y
#check add 5 -- What is the type signature of this?
```
add 5 : Nat → Nat
~~~

You _might_ have expected Lean to throw a compiler error at you, but instead 
it's given you a new function signature: `Nat → Nat`! A type which takes a
`Nat` and produces a `Nat`.

Let's see what happens when we call `add 5` as a function.

~~~admonish example title=""
```lean
def add (x y : Nat) := x + y
def addFive := add 5 -- Definition becomes fun y ↦ 5 + y
#eval addFive 8
```
13
~~~

That's new! And very powerful. And it works for any `Nat`.

```lean
#eval addFive 0 -- Returns 5
#eval addFive 2 -- Returns 7
#eval addFive 50 -- Returns 55
````

 Instead of giving you a
compiler error, if you call a function with 
_less_ arguments than required, what is returned is a _new_ function that expects the remaining
arguments to be set.

This is called function Currying after Howard Curry, who first came up with this
concept. (TODO double check)

```lean
-- This means that these two operations are the exact same!
add 5 8 
(add 5) 8
```

~~~admonish info
To expand on the  idea of currying, imagine every function is restricted to only take a single argument.
To call `add x y`, `add x` must return a function which
takes `y`,  
which in-turn returns `x + y`.

If we wanted to, we could write `add` like this:

```lean
def fun_add : Nat → (Nat → Nat) := fun x ↦ fun y ↦ x + y
```

In-fact, this is _exactly_ the original definition of `add`. Try calling `#print fun_add` and see that Lean 
prints!

This is a powerful property. `fun_add` and `add` really are teh same. Even the type signatures are the same:

```lean 
Nat -> (Nat -> Nat)
Nat -> Nat -> Nat
```

Both of these types could be read as "a function which takes two `Nat`s and returns a `Nat`", _or_ as 
"a function which takes a `Nat` and returns a function which takes a `Nat` and returns a `Nat`"! 

Neat, huh? We just omit the parentheses to be more human-readable.
~~~