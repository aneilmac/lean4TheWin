# This book is under construction

Draft Breakdown of intended chapters

1. Why you should care about lean  
   (Note: not WHAT is lean.)  
   Content:
    - Lean as a programming language
    - Lean as a solver
    - History of solvers: rocoq & for colour theorem
   Learning outcome:
    - N/A
1. Setting up lean
  - Contents:
    - Setting up lean
    - Installing VSCode
    - Installing Lean extension
    - Nagivating the infoview.
    - The `#eval` command.
  - Learning outcome:
    - How to install lean
    - Using `#print`
    - Using the infoview.
1. Expressions
   `#eval` - in depth
   - Nat type intro
   - List syntax, functional syntax.
   - Expressions, numbers
   - Tuples
1. Variables
   - Contents:
     `def` keyword
     `:=` operator (Emphasis `=` exists but is a special kind of equality)
     - variables are immutable
     - Using `#check` vs `#eval`
     - `Type`
    - Learning outcomes
      - names declarations with `def` keyword
1. Functions
   - Contents:
     - Parameters
     - `#eval` `#check` fun
     - indices, anonymous functions
     - currying
     - `fun`
     - `{}` and generics 
1. The Proposition type
   - Contents
     - What is a Proposition
     - Truthiness
     - AND/OR/IMPLIES/IMPL?
     - Type universes?
1. Proofs about propositions
   - apply 
1. Inductive type
1. Playing with Lists 
   - Content
     - Thinking of `List α` in terms of other languages
     - Choosing `List Int64`
     - `++` append function, 
     - Declaring lists `[1, 2, 3]`, `[]` for empty
     - `cons`, `1 : [4, 5, 6]` = `[1, 4, 5, 6]`
     - `reverse`
     - `xs.append` vs `append.xs`


## Thoughts

- The Rust book introduces structs before enums
- The haskell book doesn't introduce custom data types until
  chapter 8
- Functional programming in Coq introduces inductive types at chapter 1.

I think for most programmers, an understanding of struct is 
more intuitive than inductive as gentle entry. However,
inductive is a "meat and bones" type for theorem proving,
which we're trying to get to early. 

There's always the approach of the natural numbers game,
which goes straight into theorem proving - but that worked
with its own custom types.

Ideally, we have a `struct` type we can play with for 
proofs. Perhaps `Fin`? 

Intuitively the student will understand `Bool` and `Int`
have meaning without that being formally defined. 

However, we have to explain why there is a `= true` part to 
 ``!!b == b = true` if showing there is a proof.

For _very_ early theorem proving, we could look at 
function application

```
theorem my_theory {α β γ : Prop} (h : α -> β) (g: β -> γ) : (α -> γ) := by
  intro a
  apply g
  apply h
  exact a
```

Which requires us to explain `{}` brackets, which we will have to I think,
and the `Prop` "type", but is also the most functionally pure example of a 
theory using functions alone.