# It's Just a Theorem

~~~admonish warning
- [ ] Intro Curry Howard Isomorphism. 
    - Takeaway should be: if a function can be defined,that is a proof.
- [ ] Introduce `theorem five_eq_five : 5 = 5 := rfl`
- [ ] Explain type is `Prop`
- [ ]  `noncomputable` 
- [ ] Explain definition is `rfl` tactic.
- [ ] Prop vs Bool
- [ ] `=` vs `==`
- [ ] `by` keyword
- [ ] `rw` tactic applied
- [ ] `exact` tactic
- [ ] Some Nat tactics
~~~

## What is a theorem

A theorem is proposition alongside a _proof_ that the 
proposition is true.

But what is true?

### A recap of boolean truthiness

Working wither other software packages you'll be very familiar with boolean logic. Lean has a `Bool` type, which works exactly as you might expected

```lean
#check true -- (true : Bool)
#check false -- (false : Bool)

-- Bitwise AND logic
#eval true && true   -- true
#eval true && false  -- false
#eval false && true  -- false
#eval false && false -- false

-- Bitwise OR logic
#eval true || true   -- true
#eval true || false  -- true
#eval false || true  -- true
#eval false || false -- false

-- Bitwise NOT logic
#eval !true    -- false
#eval !false   -- true
#eval !!true   -- true
#eval !!false  --false

# Bitwise equality
#eval true == true   -- true
#eval true == false  -- false
#eval false == true  -- false
#eval false == false -- true
```

So far, nothing surprising: we have a `Bool` type,
and some common operators that work on Bools. The
`Bool` type is ubiqiutous in programming languages,
even in those languages that don't necessarily have types.
`Bool` is the building block on which your programs run.

You might be surprised that Lean has another "truthy" type
called `Prop`. While `Bool` is used for _execution_ (i.e. `#eval`),
and for running programs. `Prop` is used for theorem proving and _proving_
programs.

```lean 
#check True  -- (True : Prop)
#check False -- (False : Prop)

-- Note the use of `=`, and not `==`!
#check True = False -- (True = True : Prop)
```

There are some implicit conversions between `Bool` and `Prop`. If we try to 
`#eval` a Prop, it will be converted into a `Bool`!

~~~admonish example title =""
```lean
#eval True
```
true
~~~

Notice that uppercase `True` (`Prop`) has been converted to
lowercase `true` (`Bool`) when we try to _evaluate_ `True`.


Well that's confusing! You might ask: "why?"
What's wrong with `Bool`? Why do we need a different truthy-type for theorem proving?

And that's a good question!
To understand the why, 
we need to understand how solver engines like Lean work.
Fundamentally, there is a crucial assumption hidden inside 
`Bool` which can _crash_ solver engines, causing them to get stuck
indefinitely! This is known as the _Halting Problem_.

This assumption will be revealed in an exciting who-dunnit-like reveal later on. For now, you just need to understand a difference exists. 

There is a nasty _gotcha_ with boolean logic when it comes to reasoning engines like Lean.

What you may be surprised to learn is there is another
truthy-type in Lean, called `Prop`, which is more appropriate for theorem proving.

## Theorem Proving

If I ask you to prove to me `5 = 5`, how would you go about this?
It's such as _obvious_ statement, you might get utterly stuck.
It's obvious innit? Here's how we'd go about it in Lean:

```lean
theorem five_eq_five : 5 = 5 := rfl
```

Tada! A groundbreaking _proven_ theorem that `5=5`.

You might of course, not believe me. How does writing the above
line prove anything?

Lean is built on a fancy-named theory called the 
_Curry-Howard isomorphism_. This theory tells us that
a mathematical theory can be converted to a program function, 
and vice-versa.

In Lean, the Proposition `5 = 5` is the _type_ of the function. 
`rfl` is the _definition_.

Crucially, the program compiles, and the Lean infoview tells you
everything is great! In Lean, because the function compiles, 
the theory must be true. 

And that really is what theorem proving boils down to in Lean,
you write a function, the type is a proposition you want to prove,
and if you can write a function-body which compiles, then the 
theorem is true!

Here's an attempt to prove soemthing obviously false:

~~~admonish bug
```lean
theorem five_eq_six : 5 = 6 := rfl
```

Not a definitional equality: the left-hand side  
&nbsp;&nbsp;5  
is not definitionally equal to the right-hand side  
&nbsp;&nbsp;6
~~~

Now, Lean gives us a compiler error. \\(5\\) is most definitely *not* equal
to \\(6\\). We can put whatever we want in the definition, but if it
doesn't compile - then it's not a valid theorem.

~~~admonish info
The idea of _compiling_ rather than _executing_ is an important distinction here. 
`five_eq_five` only needs to compile. We would never execute the function.

In-fact, if we try `#eval five_eq_five` we'll get an error:
"cannot evaluate, proofs are not computationally relevant".
~~~

~~~admonish info
I keep referring to `theorem five_eq_five : 5 = 5 := rfl` as a _function_,
because it really is.

Cast your mind back to a function definition 

`def name : Type := ...`.

The syntax for a definition is the exact same as the definition 
for a theorem. Under the hood `theorem` will convert to 
`noncomputable def`.

`noncomputable` is a keyword that tells Lean a function is never executed,
only ever compiled. We'll be exploring `noncomputable` later on, as it's
useful for writing say, an _exact_ definition for pi, that a computer could never 
execute, but could reason about.
~~~

The question remains: what is `rfl`? 
`rfl` stands for _reflexivity_, which is theorem that
tells us that if the symbols on the left-hand side of propositional equality is equal to the right hand side, then a theorem is true!

This works for anything, as long as the symbols match exactly

```lean
theorem example_one : 23 + 5 = 23 + 5 := rfl

theorem example_two : 150 * 3 - 456 = 150 * 3 - 456 := rfl
```
]