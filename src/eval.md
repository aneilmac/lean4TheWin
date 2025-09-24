~~~admonish warning
- [x] Introduce `#eval`
- [x] Introduce `#check`
- [x] Introduce expressions: `+`, `*` `^`
- [x] Introduce `Nat` type.
- [x] Introduce `Int` type.
- [x] Introduce `String` and `++`.
- [ ] Comments
- [ ] Introduce List syntax `[1, 2, 3] ++ [4, 5, 6]`
- [ ] Introduce `List` type. 
- [ ] Introduce list built in-functions
~~~

# Good and Eval

## Using `#eval`

If you've read (and I hope you've read!), the previous section,
you should have a `.lean` file, 
and an infoview which shows the Lean elaborator at work.

We're going to learn one of the most useful statements, [`#eval`][#eval]!

When you write `#eval ...`, Lean will compile the line,
and try to _evaluate_ (a.k.a, run) the code!. 
The result will appear in the infoview. Give the following a try. You should
see the following results:

~~~admonish example title=""
```lean
#eval 3 + 4
```
7
~~~

~~~admonish example title=""
```lean
#eval 4 ^ 2
```
16
~~~


~~~admonish example title=""
```lean
#eval 18 / 3
```
6
~~~

The code we pass to `#eval` is called an _expression_. 
An expression can be as complex as you like. 
(For arithmetic, remember that
Lean follows the [order of operations][order of operations].)

~~~admonish example title=""
```lean
#eval 1 + 3 * 4 - 2
```
11
~~~

You may use parentheses to control the evaluation order.

~~~admonish example title=""
```lean
#eval (1 + 2) * 4 / 2
```
6
~~~

## Strings

Expressions are more than just numbers. 
An expression can be almost anything.
Here's a basic _string_ expression.
Strings store text. Text is written between quotes (`"`).

~~~admonish example title=""
```lean
#eval "Hello World"
```
"Hello World"
~~~

Strings in Lean are [Unicode (UTF-8)][UTF8], 
This means they can contain many different kinds of characters 
(far more than what you see on your keyboard!)

~~~admonish example title=""
```lean
#eval "Hello WØℝLΔ"
```
"Hello WØℝLΔ"
~~~

~~~admonish info
As you'll discover, special symbols are used extensively in Lean. 
The Lean VSCode extension provides a quick way to write them.

Try typing `\alpha`, in a `*.lean` file.
The word will be replaced with the `α` symbol.

In general, `\` followed by a character sequence will produce a symbol.
Here are some examples:

- `\alpha` (or `\a`) becomes `α`.
- `\beta` (or `\b`) becomes `β`. 
- `\->` and `\<-` becomes `→` and `←` respectively.
- And  [many more](./symbols.md)! 
~~~

Strings may be _concatenated_ to produce a new string with the `++` operator,
also known as 'append.'

~~~admonish example title=""
```lean
#eval "Goodbye" ++ "World"
```
"GoodbyeWorld"
~~~

Here I can use trailing/leading spaces _within_ the quotes to stop words
running together:

~~~admonish example title=""
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
To understand what this problem is, we have to understand expression _types_.

## `#check` for Types

In Lean, every expression has a _type_.
A type tells us what the expression _is_. `"Hello World"` is a `String`. 
`1 + 1 * 5` is a number.

Lean is very good at guessing what an expression's type should be, 
so we don't have to explicitly write the type out (most of the time). 
But, sometimes we need to verify the type which Lean has guessed.
How do we do this?

Introducing: the  `#check` statement! You write `#check ...` just like
`#eval`, but instead of evaluating the expression, `#check` gives you the 
_type_ of the expression.

~~~admonish example title=""
```lean
#check "Hello World"
```
"Hello World" : String
~~~

~~~admonish example title=""
```lean
#check "Hello" ++ "World"
```
"Hello" ++ "World" : String
~~~

So far, this seems fairly straightforward.
`"Hello World"` is a `String`; and `"Hello" ++ "World"` is _also_ a string,
because it results in the string `"HelloWorld"`.

So, what about numbers?

~~~admonish example title=""
```lean
#check 4
```
4 : Nat
~~~

~~~admonish example title=""
```lean
#check 5 * 5
```
5 * 5 : Nat
~~~

This makes sense too. If `4` is a `Nat`, then `5 *5` must also be a `Nat`. 
But `Nat` is a new word we have't seen before. What does `Nat` mean?

### `Nat`

`Nat` is an abbreviation of _Natural Number_. 
A Natural Number is the range of _nonnegative_ integers from zero to infinity.

\\[
\texttt{Nat} = \\{0, 1, 2, 3, 4, \dots \\}
\\]

- \\(0\\) is a `Nat`, 
- \\(1\\) is a `Nat`, 
- \\(2.5\\) is _not_ a `Nat`, because it is not an integer! 
- \\(-5\\) is also not a `Nat`, because it's less than \\(0\\).

Any whole number greater than zero is a natural number. 
There really is no upper limit for `Nat`. A `Nat` expression can 
be _any_ natural number, provided your computer has enough memory to store it.

~~~admonish example title=""
```lean
#eval 2^64 + 1
```
18446744073709551617
~~~

'That's great! I love really big numbers.' I hear you cry, 'But wait!
What about _negative_ numbers? You said a `Nat`  can't be less than zero! 
What happens with \\(3 - 4\\)?'

Let's find out with our trusty `#eval`:

~~~admonish warning title="Clamping `Nat` underflow"
```lean
#eval 3 - 4
```
0
~~~

As it turns out,
when given an expression that would result in a value less than \\(0\\), 
`Nat` expressions _clamp_ to \\(0\\). 

~~~admonish example title=""
```lean
#eval 3 - 5000
```
0
~~~

This is a unique feature of `Nat`, and not always desirable. 
After all, what if we need to work with negative numbers?

### The `Int` Type

When we need to work with numbers that can go into the negatives, instead of
`Nat`, we may use _`Int`_.  `Int` is an abbreviation of _Integer_. 

`Int` is like `Nat`, but there are an infinite number of negative
integers below \\(0\\).

\\[
\texttt{Int} = \\{\dots, {-4}, {-3}, {-2}, {-1}, 0, 1, 2, 3, 4, \dots\\}
\\]

We can see that Lean correctly chooses `Int` when we ask Lean to evaluate a
negative number.

~~~admonish example title=""
```lean
#check -3
```
-3 : Int
~~~

How do we make Lean choose `Int` 
as the type for expression `3-4`, rather than `Nat`? 

## Explicit typing

Whenever you need tell Lean that an expression has a specific type, you 
wrap the expression in parentheses, and add a `: Type` specifier. Like so:

~~~admonish example title=""
```lean
#eval ("Hello World" : String)
```
"Hello World"
~~~

~~~admonish example title=""
```lean
#eval (1 - 3 : Nat)
```
0
~~~

~~~admonish example title=""
```lean
#eval (1 - 3 : Int)
```
-2
~~~

You can add an explicit type specifier to _any_ sub-expression in the 
expression.


~~~admonish example title=""
```lean
#eval ("Hello " : String) ++ ("World" : String)
```
"Hello World"
~~~

~~~admonish example title=""
```lean
#eval (1 : Int) - (3 : Int)
```
-2
~~~

Lean is smart enough to guess that when you mix `Nat` and `Int`, 
the  expression's overall type must be `Int`.

~~~admonish example title=""
```lean
#eval (1 : Int) - (3 : Nat)
```
-2
~~~

### Bringing it all together

We have enough information to understand why `"Hello" ++ 4` failed.
`"Hello"` has type `String`, and `4` has type `Nat`. These types cannot be 
concatenated together! Append (`++`) can concatenate two strings, 
but cannot concatenate a string and a number!

This is known as _strong typing_. Lean is a strongly typed language because 
every expression has a type, and certain operations only work with certain 
combinations of types.
(This entire concept is also what lets us prove theorems!)

Later on, I'll discuss how to understand a _function_ type. Understanding 
function types will allow you to know what are the valid inputs for an
operation.

## The `def` keyword

It gets pretty repetitive writing `"Hello World"` all the time. 
Wouldn't it be nice if we could use a shorthand? Well we can, with the
`def` keyword!

~~~admonish example title = ""
```lean
def h := "Hello World"

#eval h
```
"Hello World"
~~~

`def` stands for _definition_. We are writing a new definition, named `h`. 
The expression to the right of the assignment operator (`:=`) is the definition 
of `h`.

`def` is very flexible, and can be assigned any expression you like.

~~~admonish example title = ""
```lean
def twentySomething := 5 * 5

#eval twentySomething
```
25
~~~

Just like expressions, you can add an explicit type to a 
definition by specifying the `: Type` _after_ the definition name, 
and before the assignment operator (`:=`),
like so:

~~~admonish example title=""
```lean
def p : Int := 26
```
~~~


Definitions can refer to each other and
be used in any place an expression of the 
same type is allowed:

~~~admonish example title = ""
```lean
def one := 1

def two : Nat := 2

def three := two + one

def five := 5

#eval three * five * three * five + 5
```
230
~~~

Beware the following pitfall: in Lean, the _order of declaration_ matters.
The following example fails because `c`
is declared before `a` and `b`.

~~~admonish bug title="Declaration Order"
```lean 
def c := a * b

def a := 15

def b := 6
```
Unknown identifier `a`
~~~

To fix the above snippet, I need to declare `c` **after** `a` and `b` like so:

```lean 
def a := 15

def b := 6

def c := a * b
```

This works! If we evaluate `c` we'd get \\(90\\).

Another constraint on definitions is the name of a definition must be unique.
Two definitions (in the same _scope_) cannot have the same name.

~~~admonish bug title="Duplicate definition"
```lean 
def q := "Hello World"

def q := 5
```
'q' has already been declared
~~~

This means you cannot replace or modify a definition after it has been 
created. This is referred to as _immutability_. Definitions are immutable 
(can't be changed.)

<!--
### The Bad Append

Let's consider our previous error again:

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

With our new understanding of types, we can make an educated guess as to what
went wrong here. `"Hello" ++ ?m` requires `?m` to be a `String`. 
Lean tries to guess what type the expression `4` is. From context, `4` has to be
a `String`, because that's the only type which makes sense when working with append.
But Lean doesn't know how the expression `4` could have a type `String`, and so
it throws an error.

## Wrap-up

Well done! You've learned what an expression is in Lean, and
you can use `#eval` to to evaluate expressions.

You've learned three new types: `Nat`, `Int`, and `String`, as well as
a couple of operators that work on these types.

You've learned how to explicitly define the type on an expression, and how to 
use `#check` to see what Lean things the type of an expression is.

That's a lot! Give yourself a pat on the back. Next, we're going to be looking
at _functions_.

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

#eva;



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