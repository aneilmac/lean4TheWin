


> [!WARNING]
> TODO 
> - Comments
> - Introduce `#eval`
> - Introduce expression: `+`, `*` `^`
> - Introduce `Nat` type.
> - Introduce List syntax `[1, 2, 3] ++ [4, 5, 6]`
> - Introduce `List` type. 
> - Introduce list built in-functions


# Good and Eval

## Using `#eval`

If you've read (and I hope you've read!), the previous section,
you should have a `.lean` file, 
and an infoview which shows the Lean elaborator at work.

We're going to learn one of the most useful statements, [`#eval`][#eval]!

When you type `#eval ...`, Lean will compile the line as code,
and try to run it. The result will appear in the infoview. 
Give the following a try:

~~~admonish info title=""
```lean
#eval 3 + 4
```
7
~~~

~~~admonish info title=""
```lean
#eval 4 ^ 2
```
16
~~~


~~~admonish info title=""
```lean
#eval 18 / 3
```
6
~~~

`#eval` takes _expressions_. 
These can be as complex as you like. 
For arithmetic, remember that
Lean follows the [order of operations][order of operations]. 

~~~admonish info title=""
```lean
#eval 1 + 3 * 4 - 2
```
11
~~~

You may use parentheses to control the evaluation order.

~~~admonish info title=""
```lean
#eval (1 + 2) * 4 / 2
```
6
~~~

## Strings

Of course, there's more to Lean than numbers!
So let's look at another basic type: [`String`][String]. 
Strings store text. Text is written between quotes (`"`).

~~~admonish info  title=""
```lean
#eval "Hello World"
```
"Hello World"
~~~

Strings in Lean are [Unicode (UTF-8)][UTF8], 
which means they can handle more than just the characters which 
you see on your keyboard.

~~~admonish info  title=""
```lean
#eval "Hello WØℝLΔ"
```
"Hello WØℝLΔ"
~~~

As you'll discover, special unicode symbols are used extensively in Lean. 
The Lean VSCode extension provides a quick way to write them.

Try typing `\alpha`, in a `*.lean` file.
The word will be replaced with the `α` symbol.

In general `\` followed by a character sequence will produce a symbol.
Here are some examples:

- `\alpha` (or `\a`) becomes `α`.
- `\beta` (or `\b`) becomes `β`. 
- `\->` and `\<-` becomes `→` and `←` respectively.
- And  [many more](./symbols.md)! 

Strings can be _concatenated_ to produce a new string with the `++` operator,
also known as 'append.'

~~~admonish info  title=""
```lean
#eval "Goodbye" ++ "World"
```
"GoodbyeWorld"
~~~

Here I use trailing/leading spaces _within_ the quotes to stop words
running into each other:

~~~admonish info  title=""
```lean
#eval "Hello, " ++ "Bob! " ++ "H" ++ "o" ++ "w" ++ " are you?"
```
"Hello, Bob! How are you?"
~~~

So far, so simple, right?
What happens if we write 
`"Hello" ++ 4`?

~~~admonish bug title="Type mismatch"
```lean
#eval "Hello" ++ 4
```
failed to synthesize  
&nbsp;&nbsp;OfNat String 4  
numerals are polymorphic in Lean, but the numeral `4` cannot be used in a context where the expected type is  
&nbsp;&nbsp;String  
due to the absence of the instance above
~~~

Zoinks! Lean is showing us our first _compilation error_, because
the expression contained a problem. 
Lean could not compile, because it doesn't understand how to turn `4` into a 
string. To make sense of this error message, we have to
understand what a type _is_.

## `#check` for Types

In Lean. Every expression has a type. A type tells us what an expression _is_.
I am a human. `"Hello World"` is a `String`.  `1 + 1` is a number.

Lean is very good at guessing what a type should be, so often we don't have 
to write the type of an expression explicitly. But, sometimes we need to know
the type that Lean guessed. How do we do this?

Introducing: the  `#check` statement! You write `#check ...` just like
`#eval`, but instead of evaluating the expression, `#check` gives you the 
_type_ of the expression.


~~~admonish info  title=""
```lean
#check "Hello World"
```
"Hello World" : String
~~~

~~~admonish info  title=""
```lean
#check "Hello" ++ "World"
```
"Hello" ++ "World" : String
~~~

So far, this seems fairly straightforward.
`"Hello World"` is a `String`; and `"Hello" ++ "World"` is _also_ a string,
because it results in the string `"HelloWorld"`.

So, what about numbers?

~~~admonish info  title=""
```lean
#check 4
```
4 : Nat
~~~

~~~admonish info  title=""
```lean
#check 5 * 5
```
5 * 5 : Nat
~~~

Not a gnat, but a `Nat`! 

...What's a `Nat`?

### `Nat`

`Nat` is an abbreviation of _Natural Number_. 
This is the range of _nonnegative_ integers from zero to infinity.

\\[
\texttt{Nat} = \\{0, 1, 2, 3, 4, \dots \\}
\\]

\\(0\\) is a `Nat`, \\(1\\) is a `Nat`, but \\(2.5\\) is _not_ a `Nat`, because
it is not an integer! Likewise, \\(-5\\) is also not a `Nat` either, 
because it's less than \\(0\\).

Any whole number greater than zero is a natural number. 
There really is no limit to the upper bound of `Nat`. A `Nat` expression can 
be _any_ natural number, provided your computer has enough memory to store it.

~~~admonish info  title=""
```lean
#eval 2^64 + 1
```
18446744073709551617
~~~

'That's great! I love really big numbers.' I hear you cry, 'But wait!
What about _negative_ numbers? You said a `Nat`  can't be less than zero! 
What happens with \\(3 - 4\\)?'

Well, let's find out:

~~~admonish warning  title="Clamping `Nat` underflow"
```lean
#eval 3 - 4
```
0
~~~

As it turns out,
when given an expression that would result in a value less than \\(0\\), 
`Nat` _clamps_ to \\(0\\). 

~~~admonish info  title=""
```lean
#eval 3 - 5000
```
0
~~~

This is a unique feature of `Nat`, and not always desirable. What if we need to
work with negative numbers?

### Int


Let's see what happens when we give Lean a negative number as an expression.
What type will it have?

~~~admonish info  title=""
```lean
#check -3
```
-3 : Int
~~~

An `Int`! `Int` is an abbreviation of _Integer_. 

`Int` is just like `Nat`, only there are _also_ an infinite number of negative
integers below \\(0\\).

\\[
\texttt{Int} = \\{\dots, {-4}, {-3}, {-2}, {-1}, 0, 1, 2, 3, 4, \dots\\}
\\]

Lean is smart enough to know that \\(-3\\) isn't `Nat`, so the next best guess
is \\(-3\\) must be an `Int`.

Great! So, how do tell Lean that the expression `3 - 4` should be an `Int` and 
not a `Nat`?

## Explicit typing

Whenever you need tell Lean that an expression has a specific type, you 
wrap the expression in parentheses, and add a `: Type` specifier.

~~~admonish info  title=""
```lean
#eval (1 - 3 : Int)
```
-2
~~~

You can add an explicit type to any part of the expression:

~~~admonish info  title=""
```lean
#eval (1 : Int) - (3 : Int)
```
-2
~~~

Lean is also smart enough to guess that when you mix `Nat` and `Int`, 
the  expression's overall type must be `Int`.

~~~admonish info  title=""
```lean
#eval (1 : Int) - (3 : Nat)
```
-2
~~~

<!--


So, the only logical choice for `Nat` is to _clamp_ to \\(0\\).
There is no number less than \\(0\\) in `Nat`, and there is no maximum to 
underflow to, either!

What gives? Shouldn't this be \\(-1\\)? Or heck, maybe 
even  [\\(4294967295\\)][integer underflow]? Where is \\(0\\) coming from?

The answer is that without specifying a _type_, Lean assumes that the type of 
 `3 - 4` is [`Nat`][Nat].





While we can't have a number less than \\(0\\) when working with `Nat`,
the only maximum limit is your computer's memory!
(Or the heat death of the universe, whichever comes first.)




~~~admonish info
You might be used to working with other programming languages, where implicit 
number to string conversions are common! For example, in javascript, 
`"4" + 2 = 42`, but confusingly, `"4" - 2 = 2`! In Javascript, `+` is used for
adding _and_ concatenation! And when given a choice, Javascript assumes 
concatenation first.

Lean
~~~


`+` expects _types_ which can be added together. When you add together
two incompatible types, the Lean compiler will give you an error.

~~~admonish
Try using the concat (`++`) operator to concat a `String` and `Nat`
and see what happens.
~~~






Later on in this book, I'll introduce you to more number types, 
such as `Int` and `Float`, which are not constrained in the same way as `Nat`.


<!--
## `Int` Type

'But I really want negative numbers!' I hear you cry.

No problem. Lean still has you covered, just let Lean know you want to work
with the [`Int`][Int] type.

`Int` is just like `Nat`, only there are _also_ an infinite number of negative
integers below \\(0\\).


\\[
\texttt{Int} = \\{\dots, {-4}, {-3}, {-2}, {-1}, 0, 1, 2, 3, 4, \dots\\}
\\]


To _explicitly_ get an `Int` (or a `Nat`, or any other type), we wrap
an expression in parentheses, and include a `: Type` specifier. E.g. 
`(1 : Int)`:

~~~admonish info  title=""
```lean
#eval (1 : Int) - (3 : Int)
```
-2
~~~

Lean is smart enough to know that _all_ numbers should be `Int`s, 
if the whole expression itself should be an `Int`.

~~~admonish info  title=""
```lean
#eval ((6 - 12) : Int)
```
-6
~~~

And sometimes, we don't even need to specify `: Int` at all! 
Lean automatically knows this expression should return an `Int`, because 
`-1` _must_ be an `Int`.

~~~admonish info  title=""
```lean
#eval (2^64 + 1) * -1
```
-18446744073709551617
~~~


## Comments

Now that we're comfortable with `#eval`, I'm going to take a brief intermission
to talk about _comments_.

Comments in Lean are lines that start with `--`. 
Comments are ignored by the Lean compiler.

~~~admonish info title=""
```lean
-- This line is ignored. I can type whatever I want!
#eval 1
```
1
~~~

If we have something really big to say, writing `--` every line can be pretty laborious. We can make this easier with _multiline comments_. 
Multiline comments start with `/-`, and end with `-/`

~~~admonish info title=""
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

It's worth talking about comments now, because our examples are going to
grow more complex, and comments help us understand the _why_ of an operation.





-->

[#eval]: https://lean-lang.org/doc/reference/latest/Interacting-with-Lean/#Lean___Parser___Command___eval "#eval command"
[order of operations]: https://en.wikipedia.org/wiki/Order_of_operations "Wikipedia: Order of Operations"
[integer overflow]: https://cwe.mitre.org/data/definitions/190.html "CWE: Integer Overflow"
[integer underflow]: https://cwe.mitre.org/data/definitions/191.html "CWE: Integer Underflow"
[Nat]: https://lean-lang.org/doc/reference/latest/Basic-Types/Natural-Numbers/ "Nat definition in Lean 4"
[Int]: https://lean-lang.org/doc/reference/latest/Basic-Types/Integers/#Int "Int defined in Lean 4"
[String]: https://lean-lang.org/doc/reference/latest/Basic-Types/Strings/ "String definition in Lean 4"
[python int]: https://docs.python.org/3/whatsnew/3.0.html#integers "Python Integers"
[UTF8]: https://www.w3schools.com/charsets/ref_html_utf8.asp "UTF-8 definition at w3school"