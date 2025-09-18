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

`#eval` can take any _computable_ expression. (I'll cover noncomputable
is later!) So the expression can be as complex as you like. 

Lean follows [order of operations][order of operations] as you might
expect. You may use parentheses to control the evaluation order.

~~~admonish info title=""
```lean
#eval 1 + 3 * 4 - 2
```
11
~~~

~~~admonish info title=""
```lean
#eval (1 + 2) * 4 / 2
```
6
~~~

## `Nat` Type

So far, so good. But what about negative numbers? 
The following might be a bit of a surprise:

~~~admonish warning  title="Behavior of `Nat`"
```lean
#eval 3 - 4
```
0
~~~

What gives? Shouldn't this be \\(-1\\)? Or heck, maybe 
even  [\\(4294967295\\)][integer underflow]? Where is \\(0\\) coming from?

The answer is that without specifying a _type_, Lean assumes that the type of 
 `3 - 4` is [`Nat`][Nat].

`Nat` stands for _Natural Number_. That is, the range of _nonnegative_ integers 
from zero to infinity.

\\[
\texttt{Nat} = \\{0, 1, 2, 3, 4, \dots \\}
\\]

So, the only logical choice for `Nat` is to _clamp_ to \\(0\\).
There is no number less than \\(0\\) in `Nat`, and there is no maximum to 
underflow to, either!

While we can't have a number less than \\(0\\) when working with `Nat`,
the only maximum limit is your computer's memory!
(Or the heat death of the universe, whichever comes first.)

~~~admonish info  title=""
```lean
#eval 2^64 + 1
```
18446744073709551617
~~~

```admonish
If you've worked with Python 3, you might already be used the idea of numbers 
with [no maximum limit][python int].
```

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
-->

## `String` Type

The [`String`][String] type in Lean is for storing and manipulating text. 
Strings are written between quotes (`"`).

~~~admonish info  title=""
```lean
#eval "Hello World"
```
"Hello World"
~~~

Strings in Lean are [Unicode (UTF-8)][UTF8], 
which means they accept more than just the symbols 
than you see on your keyboard.

~~~admonish info  title=""
```lean
#eval "Hello WØℝLΔ"
```
"Hello WØℝLΔ"
~~~


~~~admonish info
As you'll discover, special unicode symbols are used extensively in Lean. 
The Lean VSCode extension provides a quick way to write them.

Try typing `\alpha`, followed by a space in a `*.lean` file, 
then your text editor will replace this with the `α` symbol.
This also works for `\beta`, `\forall`, `\r`, `\and`, etc, and many others.

I have written a handy [symbols](./symbols.md) table for common unicode symbols 
and their usage!
~~~

Strings can be _concatenated_ to produce a new string with the `++` operator.

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
#eval "Hello, " ++ "Bob! " ++ "H" ++ "o" ++ "w" ++ " Are you?"
```
"Hello, Bob! How Are you?"
~~~

So far, so simple, right? 

But what happens if we write 
`"Hello" ++ 3"`? Or `1 + "World"`?:

~~~admonish bug title="Type mismatch"
```lean
#eval 4 + "Hello"
```
failed to synthesize  
&nbsp;&nbsp;HAdd Nat String ?m.1
~~~

Zoinks! Lean is complaining it doesn't know how to add a `Nat` and a `String`
type together. (We'll cover `HAdd` in more detail later.)

`+` expects _types_ which can be added together. When you add together
two incompatible types, the Lean compiler will give you an error.

~~~admonish
Try using the concat (`++`) operator to concat a `String` and `Nat`
and see what happens.
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






> [!WARNING]
> TODO 
> - Comments
> - Introduce `#eval`
> - Introduce expression: `+`, `*` `^`
> - Introduce `Nat` type.
> - Introduce List syntax `[1, 2, 3] ++ [4, 5, 6]`
> - Introduce `List` type. 
> - Introduce list built in-functions

[#eval]: https://lean-lang.org/doc/reference/latest/Interacting-with-Lean/#Lean___Parser___Command___eval "#eval command"
[order of operations]: https://en.wikipedia.org/wiki/Order_of_operations "Wikipedia: Order of Operations"
[integer underflow]: https://cwe.mitre.org/data/definitions/191.html "CWE: Integer Underflow"
[Nat]: https://lean-lang.org/doc/reference/latest/Basic-Types/Natural-Numbers/ "Nat definition in Lean 4"
[Int]: https://lean-lang.org/doc/reference/latest/Basic-Types/Integers/#Int "Int defined in Lean 4"
[String]: https://lean-lang.org/doc/reference/latest/Basic-Types/Strings/ "String definition in Lean 4"
[python int]: https://docs.python.org/3/whatsnew/3.0.html#integers "Python Integers"
[UTF8]: https://www.w3schools.com/charsets/ref_html_utf8.asp "UTF-8 definition at w3school"