# Nothing but the Truth

In Lean, they are two kinds of "truthy" types, `Bool` and
`Prop`. `Prop` is a type more suited to theorem proving,
and allows us to write _propositions_ which we can prove
using Lean's powerful theorem proving engine.

`Bool` is a type more suitable for computation. If you have 
worked in any other programming language, you will be familiar
with the `Bool` type and the recap provided here.

This chapter is a recap of booleans with respect to the `Bool` 
type. If you are comfortable with booleans already from other 
languages, you can skip this chapter entirely, or just skim it
to understand the Lean syntax.

## A recap on Boolean truthiness

A `Bool` is either

* `true`
* `false`

Lean has a boolean `not` function to swap `true` and `false`

~~~admonish example title=""
```lean
#eval not true   -- false
#eval not false  -- true
```
~~~

But it is often more convenient to use the `not` operator,
`!`:

~~~admonish example title=""
```lean
#eval !true    -- false
#eval !false   -- true
#eval !!true   -- true (e.g: not (not true))
#eval !!false  -- false (e.g: not (not false))
```
~~~

There are also  `and`, `or`, and `xor` methods. These use the
operators `&&`, `||` and `^^` respectively.

~~~admonish example title=""
```lean
#eval true  && true  -- true
#eval true  && false -- false
#eval false && true  -- false
#eval false && false -- false
```
~~~

~~~admonish example title=""
```lean
#eval true  || true  -- true
#eval true  || false -- true
#eval false || true  -- true
#eval false || false -- false
```
~~~

~~~admonish example title=""
```lean
#eval true  ^^ true  -- true
#eval true  ^^ false -- false
#eval false ^^ true  -- false
#eval false ^^ false -- true
```
~~~

## Boolean Equality

Certain types in Lean implement something boolean equality.
Types that implement boolean equality can be compared using the `==` operator, known as 'beq' or the boolean equality operator. `Nat`, `Int`, `Bool`, 
and `String` all implement boolean equality.

The boolean equality operator always returns a `Bool`. `true` if the expressions
are considered equal, or `false` if they are not considered equal.

~~~admonish example title=""
```lean
#eval true == true
```
true
~~~

~~~admonish example title=""
```lean
#eval true == false
```
false
~~~

~~~admonish example title=""
```lean
#eval 3 + 5 == 8
```
true
~~~

~~~admonish example title=""
```lean
#eval 1 == 2
```
false
~~~

~~~admonish example title=""
```lean
#eval "Cat" == "C" ++ "at"
```
true
~~~

Note that strings are _case sensitive_, that is, two strings are not considered
equal unless their capitalization matches!

~~~admonish example title=""
```lean
#eval "Dog" == "dog"
```
false
~~~

## `if` expressions

An `if` expression is always of the form 

```lean
if b then t else e
```

where `t` is the expression returned when `b` is `true`, and `e` is the 
expression returned when `b` is `false`.

Think of `if` as like a fork in the road. You go down one side of the fork
or the other, depending on `b`.

Here is an example:

~~~admonish example title=""
```lean
def helloBob name := if name == "Bob"  then "Hello, Bob!" else "You're not Bob!"
```
~~~

~~~admonish example title=""
```lean
#eval helloBob "Bob"
```
"Hello, Bob!"
~~~

~~~admonish example title=""
```lean
#eval helloBob "Rob"
```
"You're not Bob!"
~~~



If you have worked with other programming languages,
you may be more used to an `if` _statement_, which are used to modify control
flow. In Lean, `if` is an expression, which is significant for a couple of
reasons:

- The overall `if` must have a type.
- An `if`-expressions can be used anywhere an expression of the same type is valid.
- An `if` expressions must always have an `else`.
  (Otherwise what would the expression evaluate to when false?)

Here's a couple more examples:

~~~admonish example title=""
```lean
def and' (a b : Bool) : Bool := if a then b else false
```
~~~

~~~admonish example title=""
```lean
def helloName (isSteve : Bool) (name : String) := 
    "Hello, " ++ if isSteve then "Steve" else " Not-Steve!"
```
~~~
