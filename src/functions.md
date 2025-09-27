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
The operators (`++`, `+`, `^`, ...) are all functions under the hood. 
- Add (`+`) calls a function which adds two numbers together and
produces a new number.
- Append (`++`) calls a function which concatenates the 
inputs and produces a new output.

## Defining a function

Here is a simple example of a function which doubles the number you give it:

```lean
def double := fun x => x * 2
```

Here we create a definition named `double` which is a function which 
multiples the argument by \\(2\\).

The function is made of four parts:

1. The `fun` keyword. This declares we are writing a function.
1. `x` is the _argument_ (input) of the function.
1. A `=>` symbol (sometimes written: `↦` (`\mapsto`)). Everything to the right
   of this symbol is the function _body_.
1. `x*2`, the function body. States we should multiply `x` by \\(2\\).

Invoking `double` is easy. We just need to give it a number:

~~~admonish example title=""
```lean
def double := fun x => x * 2

#eval double 5
```
10
~~~

And we can use `double x` where-ever a `Nat` would be valid!

~~~admonish example title=""
```lean
#eval 2^(double 32)
```
18446744073709551616
~~~

You may have noticed that unlike other programming languages, we do not need to 
call a function with parentheses around the arguments (e.g. `double(32)`). 
In Lean, we just write the arguments directly after the function name.

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
a type. The type of `double` is:

```lean
def double : Nat -> Nat := fun x => x * 2
```

Here I have to introduce a new symbol: `->`, called 
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

As you can see, the final type in the chain of function arrows is always
the "result" of the function; everything else are the arguments **in order**.

So far, in all our examples the arguments same type. But arguments can be any 
type! For example, here is a function `pow`, which takes an `Int` 
and a `Nat`, and produces an `Int`.

```lean
def pow : Int -> Nat -> Int := fun n r => n^r
```

And if we change the order of the arguments, then we have to change the order in 
the type signature:

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

## Associativity 

Functions are _left associative_, which means arguments are evaluated 
left-to-right.

So let's say I wanted to write an expression which doubles a number, then 
doubles it again. The following example would **fail**:

~~~admonish bug title=""
```
#eval double double 6
```
Application type mismatch: The argument  
&nbsp;&nbsp;double  
has type  
&nbsp;&nbsp;Nat → Nat  
but is expected to have type  
&nbsp;&nbsp;Nat  
in the application  
&nbsp;&nbsp;double double
~~~

The reason this fails, is because Lean interprets this expression as:
`(double double) 6`! The first `double` is complaining it is 
expecting an argument of type `Nat`, but instead we are passing in an argument 
of type `Nat -> Nat` (a.k.a, the second `double`)!

To correct this, we simply add parentheses around an argument to ensure 
evaluation occurs in the correct order:

~~~admonish example title=""
```lean
#eval double (double 6)
```
24
~~~

## Named parameters

The more arguments we add to a function, the more laborious it is 
to write and to maintain. In the following example, is `y` a `Nat` or an `Int`? 
You have to carefully scan the type signature and the named arguments to match
them:

```lean
def complexFun : Nat -> Int -> Nat -> Int -> Int := fun w x y z => (x - z)^y^w
```

Luckily, in Lean, there is a better syntax called 
_named arguments_. Let's take our `double` example:

```lean
def double : Nat -> Nat := fun x => x * 2
```

This can be rewritten as:

```lean
def double (x: Nat) : Nat := x * 2
```

Both functions are exactly the same, only the latter is a more compressed
version of the first. 

Notice that when we used named parameters `fun` is implied, 
so we don't need to specify the `fun` keyword anymore. 
In addition, instead of writing the whole function type, we can get away with 
just writing the return type!

And, just like with expressions, Lean can guess an argument's type.
In the case of `double` we can omit the type of `x` (and the return type of 
`double`) entirely.

```lean
def double x := x * 2
```

Here's another, more complex example:

```lean
def sumThreeAndTriple : Nat -> Nat -> Int -> Int := fun x y z => (x + y + z) * 3
```

Using named parameters this becomes:

```lean
def sumThreeAndTriple (x : Nat) (y : Nat) (z: Int) : Int := (x + y + z) * 3
```

Note that when we have multiple parameters of the same type in a row,
we can group these together under the same explicit type:

```lean
def sumThreeAndTriple (x y : Nat) (z: Int) : Int := (x + y + z) * 3
```

This compressed syntax is very typical in Lean, and is the favoured way of 
writing functions (with a couple of exceptions we will learn about). 
Just remember that the _type_ of `sumThreeAndTriple'`
is still `Nat -> Nat -> Int -> Int`, even after we have compressed the 
definition!



<!--
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
-->