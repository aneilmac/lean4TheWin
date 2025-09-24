~~~admonish warning
- [x] Student understands `def` keyword in relation to declaration.
- [ ] Student understands basic type-specifier syntax (e.g `: Nat`.)
- [x] Student understands writing simple functions.
- [ ] Student understands they can call `[1, 2].reverse`, and
- [ ] Student understands lean file is executed in order.
- [ ]  `id` function
~~~

# Fun functions

If you've worked in another programming language, you will have come across
_functions_ (sometimes referred to as methods.)
In Lean, functions take one or more inputs, and produce an output. 

You may not have realized you've already being using functions in this tutorial! 
The operators (`++`, `+`, `^`, ...) are functions under the hood. 
- Add (`+`) calls a function which adds two numbers together and
produces a new number.
- Append (`++`) calls a function which concatenates the 
inputs and produces a new output.

## Defining a function

Here is a simple example of a function which doubles the number you give it:

```lean
def double := fun x => x * 2
```

Like we saw in the last chapter, we can assign expressions to a definition.
Here we assign a function to the definition named `double`.

The function is made of four parts:

1. The `fun` keyword. This declares we are writing a function.
1. `x` is the _argument_ (input) of the function.
1. A `=>` symbol (sometimes written: `↦` (`\mapsto`)). Everything to the right
   of this symbol is the function _body_.
1. `x*2`, the function body. States we should multiply `x` by \\(2\\).

Calling `double` is fairly easy.

~~~admonish example title=""
```lean
def double := fun x => x * 2

#eval double 5
```
10
~~~

This works for any valid `Nat`.

~~~admonish example title=""
```lean
#eval 2^(double 32)
```
18446744073709551616
~~~

You may have noticed that unlike other programming languages, I do not need to 
call a function using parentheses around the arguments (e.g. `double(32)`). 
In Lean, we don't use parentheses - We just call the function name followed
immediately by the arguments.

That's not to say we can't use parentheses! 
We may sometimes need to add parentheses around an argument to ensure 
evaluation occurs in the correct order:

~~~admonish example title=""
```lean
#eval double (double 6)
```
24
~~~

## Multiple arguments

Functions can take as many arguments as required. 
For example, to create a function which takes two arguments, we add an
additional argument between `fun` and `=>`, like so:

~~~admonish example title=""
```lean
def addAndDouble := fun x y => (x + y) * 2

#eval addAndDouble 3 4
```
14
~~~

(Note: not only do we not need parentheses to call `addAndDouble`, we also
don't need commas between the arguments!)

And we repeat the process again for three arguments...

~~~admonish example title=""
```lean
def sumThreeAndTriple := fun x y z => (x + y + z) * 3

#eval sumThreeAndTriple 3 (addAndDouble 1 1) 5
```
36
~~~

And so on!

## Function Types

Let's go back to our original example, `double`:

```lean
def double := fun x => x * 2
```

What is the type of `double`?  It must have a type because all expressions have
a type. Well guess no more, I will add it explicitly!

```lean
def double : Nat -> Nat := fun x => x * 2
```

To do this I have to introduce a new symbol: `->`, 
the _function arrow_ (sometimes written as `→` (`\rightarrow`)).

The function arrow tells us that the type of `double` is an expression that
takes a `Nat` and produces a `Nat`.

The more arguments a function has, the more function arrows you need:

```lean
def sumThreeAndTriple : Nat -> Nat -> Nat -> Nat := fun x y z => (x + y + z) * 3
```

We read the type of `sumThreeAndTriple` as a function that 

1. Takes a `Nat`.
1. Takes a `Nat`.
1. Takes a `Nat`.
1. Produces a `Nat`.

The final type in the chain is the "result" of the function, 
while everything else are the arguments **in order**.

Not everything has to be the same type, of course. 
For example, here we define a function `pow` to be a function
which takes an `Int` and a `Nat`, and produces an `Int`.

```lean
def pow : Int -> Nat -> Int := fun n r => n^r
```

We could even swap around the order of arguments if we wanted:

```lean
def pow' : Nat -> Int -> Int := fun r n => n^r
```

~~~admonish info
A little functional programming style-tip: functional programmers sometimes
append the prime symbol (`'`), at the end of a function name, to show that the
function is a variation on some other similarly-named function. 
`'` is a valid symbol to use in a definition name, and can go anywhere. Like so:

```lean
def o'keeffe's := 42
```
~~~

## Named parameters

It becomes obvious very quickly that functions are very laborious to write out!


```lean
def sumThreeAndTriple' : Nat -> Nat -> Int -> Int := fun x y z => (x + y + z) * 3
```

The more arguments we add, the more difficult the function becomes to parse,
and the more we lost track of what is going on. Luckily, in Lean, there is a 
much shorter syntax:

```lean
def sumThreeAndTriple' (x : Nat) (y : Nat) (z: Int) : Int := (x + y + z) * 3
```

Much better! Both function are exactly the same, only we use _named_ parameters
instead. Named parameters are placed between the name and the type of the 
function definition.

Notice that when we used named parameters `fun` is implied, 
so we don't need to specify this keyword anymore. 
In addition, we write the type of the function 
only in terms of the return value!

We can shorten this function _even further_. When you have several 
named parameters of the same type in a row, you can group them together.

```lean
def sumThreeAndTriple' (x y : Nat) (z: Int) : Int := (x + y + z) * 3
```

This is the typical syntax for functions in Lean, as it's much easier to
read and reason about. Just remember that the _type_ of `sumThreeAndTriple'`
is still `Nat -> Nat -> Int -> Int`, even after we have compressed the 
definition!


## Functions

`def` can also be used to define _functions_. You can add arguments like so:



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