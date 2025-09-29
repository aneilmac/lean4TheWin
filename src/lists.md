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

We've been looking at some advanced concepts, but so far, 
all our examples have been working with `Nat`, `Int`, `String` and `Bool`. 
Before we jump into theorem proving, it's worth looking at another useful type.


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

Let's see what happen when we check `List` type.

~~~admonish example title=""
```lean
#check List
```
List.{u} (α : Type u) : Type u
~~~

From reading the pervious chapters, you will be familiar with the syntax here.
But it's a new combination of things we've seen!

`List` takes a type argument `α`. And the universe level 
of `List α` is dependent on the universe level of type `α`.

We should read `List α`, to mean a List of
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
#check List.filter
```
List.filter {α} (p : α → Bool) (l : List α) : List α
~~~

We should read this as `List.filter` takes a filter function `p : α → Bool`
that returns `true` or `false`, and an input list `l : List α`.
All elements in `l` are tested against `p`, and the newly returned list 
contains only those elements in `l` where `p` returns true.

Let's quickly consider the type signature of `List.filter`:

~~~admonish example title=""
```lean
{α : Type u} → (α → Bool) → List α → List α
```
~~~

We've not seen this type signature before, 
you should beware that this is very different to

~~~admonish bug title=""
```lean
{α : Type u} → α → Bool → List α → List α
```
~~~

- The former type signature takes two args: a `(α → Bool)` and a `List α`.
- The latter type signature takes three args: a `α`, a `Bool`, and a `List α`! 

The parentheses is important! It completely changes the meaning.

So here's a couple of example usages of `List.filter`:

Here, we only return true if `x == 0`, so we should only see a list of `[0]`

~~~admonish example title=""
```lean
#eval List.filter (fun x => x == 0) (List.range 10)
```
[0]
~~~

Here, we always return false, meaning we should only ever see an empty list:

~~~admonish example title=""
```lean
#eval List.filter (fun _ => false) (List.range 100)
```
[]
~~~

And the opposite, returning true, means both lists are always the same:

~~~admonish example title=""
```lean
#eval List.filter (fun _ => true) (List.range 10)
```
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
~~~

The `id` function as we know just returns the argument it's given, so if we
use this on a list of booleans, we'll only get the `true` elements back:

~~~admonish example title=""
```lean
#eval List.filter id [true, false, true, true, false, false, false]
```
[true, true, true]
~~~

We asked earlier, how to _only_ get those elements which were multiples of 
\\(5\\) or \\(7\\).

~~~admonish info
From other programming languages you may be familiar with the `%` operator,
know as modulo. Modulo tells us the _remainder_ of a division when working
with integers. `7 / 5` is \\(1\\) (rounded down), 
and `7 % 5` is \\(2\\) (remainder of `7 / 5`).
~~~

~~~admonish example title=""
```lean
#eval List.filter (fun x => x % 5 == 0 || x % 7 == 0) (List.range 100)
```
[0, 5, 7, 10, 14, 15, 20, 21, 25, 28, 30, 35, 40, 42, 45, 49, 50, 55, 56, 60, 63, 65, 70, 75, 77, 80, 84, 85, 90, 91,
 95, 98]
~~~

## Partial function application and currying

Recapping what was said earlier, imagine we have a function:
which takes some `α`, some `β`, and produces some `γ`:

```lean
α -> β -> γ
```

This is very different from this type signature
which takes an `(α -> β)`, and produces some `γ`.

```lean
(α -> β) -> γ
```

But what about _this_ variant:

```lean
α -> (β -> γ)
```

Here we have a function that takes an `α`, and returns a function of the form
`(β -> γ)`.

You may be surprised to learn that `α -> (β -> γ)`, and `α -> β -> γ` **are the
same type signature**!

Here's an example.

~~~admonish example title=""
```lean
def add : Nat -> Nat -> Nat := fun x y => x + y
#check add 5 -- What is the type signature of this?
```
add 5 : Nat → Nat
~~~

You _might_ have expected Lean to throw a compiler error at you 
(most programming langues would), but instead Lean has given you a new function 
signature: `Nat → Nat`! A type which takes a `Nat` and produces a `Nat`.

~~~admonish example title=""
```lean
def add : Nat -> Nat -> Nat := fun x y => x + y
def addFive := add 5
#eval addFive 8
```
13
~~~

In the new function `addFive`, it's like Lean has stored our original argument,
and is now waiting for the remaining arguments to complete the function 
signature!

```lean
#eval addFive 0 -- Returns 5
#eval addFive 2 -- Returns 7
#eval addFive 50 -- Returns 55
```

This is called function Currying after Howard Curry, who first came up with this
concept. Instead of thinking of a function as taking multiple arguments, in 
Lean, every function really takes just a single argument, and returns a new
function which takes the remainder.

This means that the following two calls are equivalent:

```lean
add 5 8 
(add 5) 8
```

And the definition of `add` could be rewritten as:

```lean
def add' : Nat → (Nat → Nat) := fun x => fun y => x + y
```

There's absolutely no difference in calling it!

~~~admonish example title=""
```lean
#eval add' 5 8
```
13
~~~

This is an incredible feature of Lean, in that we can use partially applied 
functions to create new functions. Let's go back to our filter example.

Previously, we showed filter function which would always return `true`,
producing the original list

~~~admonish example title=""
```lean
#eval List.filter (fun _ => true) (List.range 10)
```
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
~~~

This can be rewritten in terms of `Function.const: α -> β -> α` (you should
remember `const` from the previous chapter).

~~~admonish example title=""
```lean
#eval List.filter (Function.const true) (List.range 10)
```
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
~~~

Here, `Function.const true : β -> Bool`, perfectly fits the type signature 
of `p : Nat -> Bool`. Lean's powerful type guessing will set `β` to `Nat` for 
us.

Here's another example. Let's consider we have a function on `List Nat`, which
produces a new list, returning only those numbers which are even:

```lean
def evens (l : List Nat) : List Nat := List.filter (fun x => x % 2 == 0) l
```

We can take advantage of the fact of partial function application on 
`List.filter`, to rewrite this as:

```lean
def evens' := List.filter (fun x => x % 2 == 0)
```

Here, `evens'` is defined as a partial application `List.filter`. 
When the final input parameter is provided, you'll get the even-number list!

## The `map` function

Let's say we want to convert a number to a string. We would use the `toString`
function.

~~~admonish example title=""
```lean
#eval toString 4
```
"4"
~~~

So how do we change a list of numbers to a list of strings?

This is where we can use `List.map`

~~~admonish example title=""
```lean
#check List.map
```
List.map {α β} (f : α → β) (l : List α) : List β
~~~

`List.map` takes a function `f: α → β`, and a list `l : List α`, and produces
a new `List β`. In effect, `List.map` applies `f` to each element in `l`, and 
returns the output as a new list. We can slot `toList` in there easily.

~~~admonish example title=""
```lean
#eval List.map toString (List.range 10)
```
["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
~~~

`map` is a very flexible function, and allows for any function expression
that confirms to the type signature. As you can see the type `β` can be derived
entirely from the signature of `f`.

~~~admonish example title=""
```lean
#eval List.map fun (x => x % 2 == 0) (List.range 10)
```
[true, false, true, false, true, false, true, false, true, false]
~~~

~~~admonish example title=""
```lean
#eval List.map (Function.const 7) (List.range 10)
```
[7, 7, 7, 7, 7, 7, 7, 7, 7, 7]
~~~


<!--

The first question you might be asking yourself is: what use is knowing any of
this? I understand `add` has a type signature of `Nat -> Nat -> Nat`? But what
practical use is that to me?

Having a strong understanding of function arrows is important for theorem
proving. It also helps explain to use a surprising behavior of Lean!





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