~~~admonish warning
- [x] Comments
- [x] Introduce List syntax `[1, 2, 3] ++ [4, 5, 6]`
- [x] Introduce `List` type. 
- [ ] Introduce list built in-functions

-"Comments" section disjoined, move elsewhere.
- Functions rushed/clunky. Perhaps put function defs here.
- Does the student really need to know `["Hello", 4]` and other error examples?

TODO: 
- Introduce functions with List.range.
- Introduce functions with multiple arguments with List.range'
- Introduce the idea of `'` (prime) as an "alt" function
name. Will help in future.

~~~

# Let me List the Ways

So far you've being doing great! You've learned about `#eval`, expressions,
types, and the syntax of functions in Lean. 

But so far, I've been limiting you to only working with `Nat` and `Int`, with
a couple of `String` examples.

This chapter we're going to cement our understanding of functions by 
introducing a new in Lean: `List`.

A [`List`][list] in lean is a collection of things. 
The collection is _ordered_. It has a start and an end.

Lists are declared by putting comma separated elements between square brackets 
(`[]`). For example:

~~~admonish example title=""
```lean
#eval [1, 2, 3, 4, 5]
```
[1, 2, 3, 4, 5]
~~~

Just like [`String`][String], append (`++`) also works on Lists to produce a new
list from the concatenation of two lists.

~~~admonish example title=""
```lean
#eval [1, 2, 3] ++ [4, 5, 6]
```
[1, 2, 3, 4, 5, 6]
~~~

~~~admonish example title=""
```lean
#eval [-1] ++ [-1] ++ [-1] ++ [-1] ++ [-1] ++ [-1]
```
[-1, -1, -1, -1, -1, -1]
~~~

Lists also have an additional operator `::` called _cons_, 
for adding an element to the beginning of a list:

~~~admonish example title=""
```lean
#eval 1 :: [2, 3, 4]
```
[1, 2, 3, 4]
~~~

You can use multiple `cons` 

~~~admonish example title=""
```lean
#eval 1 :: 8 :: [100, 130, -3]
```
[1, 8, 100, 130, -3]
~~~

In fact, `[1, 2, 3]` is the _exact same_ as writing `1 :: 2 :: 3 :: []`, with 
`[]` being the _empty list_.

~~~admonish example title=""
```lean
#eval 1 :: 2 :: 3 :: []
```
[1, 2, 3]
~~~

## List Type

We are always interested in types in Lean!
What is the type of list?
Well it actually changes _based on the
contents of the list._

~~~admonish example title=""
```lean
#check ["Apple", "Orange", "Banana"]
```
["Apple", "Orange", "Banana"] : List String
~~~

Here, we read `List String` as a list of strings.

~~~admonish example title=""
```lean
#check [1, 2, 3]
```
[1, 2, 3] : List Nat
~~~

Here, we read `List Nat` as a list of natural numbers.

Lists in lean are _homogeneous_, they can only ever contain expressions of a
single type.
(Just like how you might expect a shopping list to contain only shopping - and not, say, the names of former U.S. presidents.)

When we read the type `List α`, we read it as a List of
elements of type `α`. The list can _only_ contain elements
of type `α`.

For example, the following will fail:

~~~admonish bug title="Heterogeneous list error"
```lean
def myList := [1, 2, 3]

#check ["A", "B", "C"] ++ myList
```
failed to synthesize  
&nbsp;&nbsp;HAppend (List String) (List Nat) ?m.8
~~~

`["A", "B", "C"]` is a `List String`, and `myList` is a 
`List Nat`. Lean does not know what the output `List α`
should be, and borks out!

Here is another example of Lean failing to guess an appropriate `α`:

~~~admonish bug title="Ambiguous empty list"
```lean
#eval []
```
don't know how to synthesize implicit argument 'α'  
&nbsp;&nbsp;@List.nil ?m.2  
context:  
&nbsp;&nbsp;⊢ Type ?u.730
~~~

Here Lean tells us it can't guess what type `α` should be. 
To fix this, we just need to provide Lean an explicit argument 
for `α`.

~~~admonish example title=""
```lean
#eval ([] : List Int)
```
[]
~~~

~~~admonish info
It may seem a bit strange that Lean needs to know the full type of `[]` to 
be able to evaluate `[]`. 
After all, an empty list is just an empty list, right?

Yes! And in theorem proving, we likely could get away with this, but
`#eval` requires a type to be _computable_.
In Lean, not all types are computable, and Lean needs proof that
the contents of a list _can_ be evaluated, even when the list is empty!
~~~

## A note on namespaces

Namespaces are a way of neatly grouping related functions together, so that they
are easy to find and access. 

To access a function in a namespace we write out the name of the namespace,
followed by a dot (`.`), followed by the function name. E.g. 
`Namespace.function`.

Often, when we have functions related to a type (e.g. `Nat` or `List`), the 
namespace is the type's name.

So for example, instead of writing `(3 + 5 : Nat)`, we could write, 
`Nat.add 3 5`. `add` is the function, and `Nat` is the namespace that `add`
lives in! (In fact `Nat.add` _is_ the function Lean calls when we write
`3 + 5`!)

~~~admonish info
Previously I said that you cannot have two definitions with the same name, but
this isn't entirely true. You can have definitions with the same name, if they
live in separate namespaces! This is why both an `Int.add` and `Nat.add` exist. 
~~~

With the VSCode lean extension try writing `List.` then hit `Ctrl+Enter` on your
keyboard. You'll see the _autocomplete_ suggest all the theorems and functions
found in the `List` namespace! You can scroll the suggestions with the up and
down arrow keys. You can inspect the type signatures of these functions, and
read the documentation provided to try and make sense of them.

But often, this can be overwhelming, and we haven't even
covered the theorems in the `List` namespace yet! 
For now, let's focus on some of the simpler `List` functions. 

## Range

If I were to type out a list containing the numbers \\(1\\) 
to \\(100\\), (`[1, 2, 3, 4...]`) I'd probably give up.
Luckily, there is a function that does this for us called `List.range`

~~~admonish example title=""
```lean
#check List.range
```
List.range (n : Nat) : List Nat
~~~

You should be able to understand this type signature 
after reading the previous chapter. `List.range` takes a `Nat`, and produces
a `List Nat`.


~~~admonish example title=""
```lean
#eval List.range 100
```
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
 92, 93, 94, 95, 96, 97, 98, 99]
~~~

Ah, but wait a second! `List.range` counts _ordinaly_ it counts 100 numbers starting from \\(0\\). 
I wanted to start from \\(1\\)! Luckily there is a variant: `List.range'`,
which takes as the first parameter, the starting number.

~~~admonish example title=""
```lean
#eval List.range' 1 100
```
[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
 92, 93, 94, 95, 96, 97, 98, 100]
~~~

## Filter

Let's say we wanted a list of all numbers, from `1` to `100`, but we wanted
only those numbers that were multiples of \\(7\\) or \\(5\\). How might we do
this?

There is a `List` function which allows us to filter the contents of a list. 
Let's take a look.

~~~admonish example title=""
```lean
#check List.range
```
List.range (n : Nat) : List Nat
~~~

List.filter.{u} {α : Type u} (p : α → Bool) (l : List α) : List α



<!--

~~~admonish example title=""
```lean
#eval [2, 4, 6, 8] ++ [] -- What do you think this will do?
```
[2, 4, 6, 8]
~~~

If you guessed that you'll just get back your original list, well done!

Now let's see what happens if we try to 
`#eval []` the empty list on its own.


Uhoh! Another compiler error. Again, Lean wasn't 
able to guess the type of the expression. 
But what's an implicit argument? And where's this `α`
come from?

### Types of List

To understand why `Lean` is having trouble with
an empty list, we must understand what is the type
of a list expression.

~~~admonish example title=""
```lean
#check [1, 2, 3]
```
[1, 2, 3] : List Nat
~~~

This is new, instead of one word we get _two_: `List`
and `Nat`.


Here's another example using elements of type `String`:

~~~admonish example title=""
```lean
#check ["Apple", "Orange", "Banana"]
```
["Apple", "Orange", "Banana"] : List String
~~~

Just like `"Hello" ++ 4`, Lean will fail to evaluate
`["Hello", 4]`, and we'll get a similar error:

~~~admonish bug title="Heterogeneous list error"
```lean
#eval ["Hello", 4]
```
failed to synthesize  
&nbsp;&nbsp;OfNat String 4   
numerals are polymorphic in Lean, but the numeral `4` cannot be used in a context where the expected type is   
&nbsp;&nbsp;String  
due to the absence of the instance above
~~~

Likewise, Lean cannot append two lists of different types either! 
Because then, what would the type of the resulting expression be?

And _this_ is why `#eval [2, 4, 6, 8] ++ []` works, and `#eval []` doesn't!
Lean _knows_ that the empty List in the former expression must be a 
`[] : List Nat`

But Lean doesn't know the type of `[]` on its own, we can run 
`#check []` to demonstrate this:

~~~admonish example title=""
```lean
#check []
```
[] : List ?m.1
~~~

`?m.1` is a _placeholder_ in the infoview. Any type that starts with `?` is a
placeholder. 

The placeholder tells us Lean knows there should be a type here, but doesn't
know what it should be. `?m.1` could be `Int`, or `String`, or a 
U.S President.

~~~admonish info
It may seem a bit strange that Lean needs to know the full type of `[]` to 
be able to evaluate `[]`. 
After all, an empty list is just an empty list, right?

And you may well be right! But Lean is very conservative when it comes to types. In Lean, not all types can be evaluated, and Lean needs proof that
the contents of a list _can_ be evaluated, even when the list is empty!
~~~


To solve our `#eval []` problem, 
all we've got to do is tell Lean what the list is!

~~~admonish example title=""
```lean
#eval ([] : List Int)
```
[]
~~~

Tada! That was an easy fix.

## Functions and lists

We've spent a lot of time looking at types and operators, but we've not called
a single function yet. So let's fix that.

~~~admonish example title=""
```lean
#eval List.reverse ["A", "B", "C"]
```
["C", "B", "A"]
~~~

If you're familiar with programming in other languages (especially those in the _C_ family), this syntax may seem foreign to you.

`List.reverse` is a _function_.  _reverse_ is the name of the function, 
and `List` is the namespace of the function. As you might have guessed,
`reverse` reverses the order of elements in a list!

A namespace is collection of functions, types, and theorems helpfully grouped 
together under a common heading.
As you might have guessed, `List` is a namespace in Lean which contains common
to List!

~~~admonish info
If you type `#eval List.` into a `*.lean` file, followed by keyboard shortcut 
`Ctrl+Space`, you can see all the functions/theorems in `List` recommended by 
Lean within the namespace.
~~~

Let's look at another function in `List`, `append`

~~~admonish example title=""
```lean
#eval List.append ["A"] ["B"]
```
["A", "B"]
~~~

As you might have guessed, `List.append` does exactly the same as the append
operator `++`. In-fact `++` just calls `List.append` behind the scenes!

In _C_-style languages, an equivalent function may be

~~~admonish example title="C-style function call"
```cs
List.append(["A"], ["B"])
```
~~~

Not-so in Lean. 
In Lean there is _no_ opening/closing parentheses to call the function, 
and we do not separate multiple arguments with commas.

The arguments of a function can be any valid expression. When there is 
ambiguity, we can use parentheses around an argument.

~~~admonish example title=""
```lean
#eval List.append (List.reverse ["C", "B", "A"]) ["D", "E", "F"]
```
["A", "B" "C", "D", "E", "F"]
~~~

Try exploring the following functions:

| Function | Description | Example |
|-|-|-|
| `List.range` | Generates a list from 0 to the given number | `List.range 100` |
| `List.range'` | Like `range` bust starts on a specific number | `List.range' 2 100` |
| `List.length` | Return the number of elements in a list. | `List.count [1, 2, 3]`|
| `List.head!` | Returns the head (a.k.a first) element of a list. | `List.head! [1, 2, 3]` |
| `List.tail!` | Returns a list of everything _except_ the head of the list. | `List.tail! [1, 2, 3]` |

[List]: https://lean-lang.org/doc/reference/latest/Basic-Types/Linked-Lists/ "List defined in Lean 4"
[String]: https://lean-lang.org/doc/reference/latest/Basic-Types/Strings/ "String definition in Lean 4"


<!--
## Comments

First: a brief intermission to talk about _comments_.


Comments in Lean are lines that start with `--`. 
Comments are ignored by the Lean compiler.

~~~admonish example title=""
```lean
-- This line is ignored. I can type whatever I want!
#eval 1
-- This line is also ignored!
```
1
~~~

If we have something really big to say, 
we may use _multiline comments_. 
Multiline comments start with `/-`, and end with `-/`.
Everything in-between is ignored!

~~~admonish example title=""
```lean
/-
SHOPPING LIST:
* Eggs (x12)
* MILK (2 pints)
* CANDLES (x300)
-/
#eval 1 + 1
```
2
~~~

As this tutorial advances, I'll add comments
to talk you through the steps line-by-line, as needed.

## List
-->