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

What is the type of list? This changes based on the
contents of the list.

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

If we try a `#check List` we get a funny type signature:

~~~admonish example title=""
```lean
#check List
```
List.{u} (α : Type u) : Type u
~~~

In later chapters I will cover the meaning of `{u}` and `Type u`,
which have special meaning in lean's type system.

For now, let's pretend these elements don't exist and hone in on the following part:

```lean
List α
```

If you think `α` looks similar to a named argument in a function
declaration, you'd be right! `List` is a function that takes 
an argument `α`, with `α` being the _type_ that a `List` can hold.

This is also why the following code fails:

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

~~~admonish bug title="Another heterogeneous list error"
```lean
#check ["A", "B", "C"] ++ [1, 2, 3]
```
failed to synthesize  
&nbsp;&nbsp;OfNat String 1  
numerals are polymorphic in Lean, but the numeral `1` cannot be used in a context where the expected type is   
&nbsp;&nbsp;String  
due to the absence of the instance above
~~~

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