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

Over the past couple of chapters I have been obsessively talking about types.
And there was a reason we had to become comfortable with types to be able to
prove things in Lean.

Lean is built on the Curry-Howard isomorphism. To summarize: a theorem can be
converted to a function. If the type of the function is the _proposition_, then
having a definition for that function, would be the _proof_ that the proposition 
is true.

## The `Prop` type

Previously we've learned about the existence of universes, or sorts in Lean.

We understand that all our "common" or computable types live in the `Type` 
universe (`Sort 1`). And that `Type` lives in `Type 1`, and `Type 1` lives in
`Type 2` and so on.

So far, this has been of little practicle use! If all our types live in `Type`,
then all we have to care about is `Type`!


But now we enter theorem proving, we need to know about `Prop`.
`Prop` is an alias for `Sort 0`, the lowest possible universe level in Lean. 
Much like you have types in `Type`, you have propositions in `Prop`. Here's
some examples:

~~~admonish example title=""
```lean
#check True
```
3 = 3 : Prop
~~~

Don't be fooled into thinking this is the same as `3 == 3`!
`3 == 3` is a _computable_ operation that results in `(true : Bool)`.

`3 = 3` (note the single equals), is a _proposition_ that \\(3\\) is equal to
\\(3\\). Think of it more like a type. Let's try proving it.

## Theorem Proving

Here is a very simple Lean proof:

```lean
theorem three_eq_three : 3 = 3 := rfl
```

Tada! A groundbreaking _proven_ theorem that `3 = 3`.

If you squint, you can see how this is similar to how we use `def name := ...`.

We have `three_eq_three` which is our theorem name, `3 = 3`, which is our type,
and `rfl` which is our proof that `3 = 3`. (I'll explain `rfl` later!)

Crucially, the program compiles, and the Lean infoview tells you
everything is great! Because the function is valid, then theory must be true. 

Let's consider another prop:

~~~admonish example title=""
```lean
#check True
```
3 = 5 : Prop
~~~

`3 = 5` is demonstrably **false**! 
But being a false statement doesn't mean `3 = 5` isn't a valid proposition! 
Propositions can be true _or_ false. What confirms a proposition true is the 
_proof_ a.k.a the function definition. You'll see we can't prove `3 = 5` the
same way we proved `3 = 3`:

~~~admonish bug
```lean
theorem three_eq_five : 3 = 5 := rfl
```
Not a definitional equality: the left-hand side  
&nbsp;&nbsp;3  
is not definitionally equal to the right-hand side  
&nbsp;&nbsp;5
~~~

The false theorem doesn't compile, and yo get an error message.

Now, bear in mind, an error message **does not mean a theory is false**.
An error message _only_ means that the theory is not valid. To prove `3 = 5` 
is false, we would require a proof of the following proposition:

~~~admonish example title =""
```lean
#check 3 ≠ 5 
```
3 ≠ 5 : Prop
~~~

Which we will cover in later chapters about contradictions and falseness.

## The `rfl` proof

If I asked you to prove that `3 = 3`, it's likely you'll get stumped by this
question.

After all, isn't it obvious? Isn't the proof just "well _look_, how can it 
not be?" What more do you expect from me?

`rfl` stands for _reflexivity_. Reflexivity is a fancy way of saying that 
for any, `a`, `a = a` is true. It is the definition of equality in Lean.

`rfl` is more powerful than you might think. `a` can be as complex a statement
as you like, as long as both sides are equal, then the proposition must be true
by reflexivity. 


```lean
theorem example_one : 23 + 5 = 23 + 5 := rfl

theorem example_two : 150 * 3 - 456 = 150 * 3 - 456 := rfl
```

