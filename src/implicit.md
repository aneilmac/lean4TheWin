# Implict Implications

Pop-quiz! What happens when I write:

```lean
#check 4
```

You know this one! The Lean infoview returns:

~~~admonish example title=""
```lean
#check Nat
```
4 : Nat
~~~

Because \\(4\\) is a member of type `Nat`.

But what might you expect if I write?

```lean
#check Nat
```

Will Lean throw a compilation error? Will it return something? Let's see:

~~~admonish example title=""
```lean
#check Nat
```
Nat : Type
~~~

That's a bit of a head-scratcher isn't it. The type `Nat`, seems to be an 
expression with type `Type`.

Well let's go one step further:

~~~admonish example title=""
```lean
#check Type
```
Type : Type 1
~~~

Uh...

~~~admonish example title=""
```lean
#check Type 1
```
Type 1 : Type 2
~~~

Hold on...


~~~admonish example title=""
```lean
#check Type 2
```
Type 2 : Type 3
~~~

What is going on here?

What you are seeing is one of the most powerful aspects of Lean. Every `Type`
is an expression, and since expressions have types, then the type must itself 
have a type!

Take a moment to digest this.
It _really_ is turtles all the way down. There is an infinitive universe of
types in Lean. You'll never run out. `Nat` is a member of `Type 0` (shortened to
just `Type`), which is a member of `Type 1`, which is a member of `Type 2`, 
and so on, forever.

In this tutorial, all types you'll encounter 
(including `Nat`, `Int`, `String`, etc.) will be members of `Type 0` (or 
`Sort 0`, which is a special type universe for theorem proving we will get to.)

## Types are expressions (really)

I's important to understand that types are themselves expressions, and can be
used as such.

