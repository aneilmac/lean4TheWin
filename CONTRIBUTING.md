# Meta 

## Overview

The aim of this project is a textbook for Lean 4, in the style of 
[Learn You a Haskell for Great Good](https://learnyouahaskell.com/chapters).

The target audience are programmers interested in unlocking lean's proof-solving
abilities, to help with their own problem solving. They are not expected to 
have a strong mathematical background.

The audience will primarily be self-directed learners as such:
  - Proof-solving must be introduced early. The ability to be able to prove 
    things is a main motivator.
  - Tools should be introduced early to allow for debugging / self-correction
    when stuck.
  - No problem should force the student to "hunt elsewhere" due to missing 
    context.
  - Complexity/difficulty should rise slowly. The student should not lose 
    motivation because the text is "too hard."

## Learning Outcomes

- Student can create a lean file in vscode. Student can install lean extension.
- Student understands and can interpret lean infoview.
- Student knows custom symbols are input using `\`, and are familiar with:
  `\forall`, `\exists`, `\<-`, `\.`, `\alpha`, `\beta`.
- Student know how to find/reason about symbols they are unfamiliar with.
- Student understands lean file is executed in order.
- Student can perform evaluation using `#eval`, `#check` and `#print`.
- Student has the ability to create own inductive functions, and understands the 
  **style** difference between parameterized and non-parameterized types.
- Student understands `fun` and anonymous function types.
- Student understands partial function application and currying.
- Student understands the concept of a noncomputable function.
- Student has ability to create their own types (inductive and structural.)
  - Can make custom constructors
  - Can have indexed/non-indexed parameters 
    (Note: does not need to appreciate practical difference, only that they exist)
  - Can use namespaces to create scoped functions
- Student has the ability to apply common typeclass instances to their type.
    (Eq, ToString, etc.)
- Student has the ability to create their own theorems using the following:
    - `intro` and `rintro`.
    - `rw [x] rw [<- x]`, `rw [x, y, ...]`, `rw at`.
    - `induction`, `induction using`, `induction generalizing`
    - `apply` and `apply at`
    - `exact`
    - `exists`
    - `simp`, `simp [x]`, `simp at`, `simp only`, `simp?`.
    - `apply` and `apply at`. Understands goal application is "backwards."
    - `by_cases, rcases, rintro`. 
    - `exists`
    - `by_contra`, `exfalso`.
    - `conv`
    - `sorry`
    - (Mathlib) `ring` and `omega`
- Student understands named parameters are equivalent  to `\forall`.
- Understands the dangers of `sorry` and why it gives a warning.
- can manipulate using `symm`, `f.mp`, `f.mpr` `mt`.
- Understands the `And` and `Or` types, and some theorems related to them.
- Understands the `List`, `Nat` types and how they are implemented.
- Understand how to use `rfl?`, `simp?` `#loogle` and source code, API docs to
  solve proofs and search for missing steps in proofs.
- Understands difference between propositional and boolean equality.
- Understands that there exists classical/constructivist logic, and understands
  that classical logic is related to the halting problem in relation to the lean
  solver.
- Understands `Sort 0`, `Sort n`, `Type`, `Type n` and the concept of type 
  universes.
- Student can give a basic answer to the question:  What is the Curry-Howard 
  isomorphism?
- Student is aware they can create their own typeclasses.
  (Note: does not need to know how, that can be further reading)

### Style guide

#### Prose

- This textbook aims to be informal and whimsical. 
- Authorial tone: excited. Lean is _groundbreaking_. The ability to prove things
  is _incredible_.
- Assume audience are enthusiastic software developers with a High-School level 
  Maths knowledge.
  - Expect the user to be very familiar with lists and list manipulation (e.g 
    sorting algorithms, types, and functions such as `reverse`.)
  - Do **not** expect the user to be familiar with the concepts of rings / 
    propositional logic, or functional programming style.
  - When a term in mathematics and computer science conflict, clarify using the
    computer science term as the baseline. (E.g. when introducing `Int`, start
    from the baseline assuming the user will interpret this as an `Int64`.)
  - Mathematical concepts should be **informally** explained if they are to be 
    introduced. 
- The aim is to introduce a topic, not necessarily explain it in detail. 
  We _want_ the user to know just enough to be dangerous. 
- Examples are practical, and the student wants to code. User should always be 
  able to follow along in their own editor as they go.
- When in danger of being bogged down in detail, introduce other lean resources 
  for further reading.
- For accessibility purposes prefer simple English where appropriate.
- Avoid dense passages. Passages should be broken up with code snippets,
  diagrams, and illustrations.

#### Code

- Favour short, simple snippets, over longer functions, even if this means
  more convoluted proof steps.
- The best tutorial code can be copy+pasted directly into
the consumer's text file.
- Avoid relying on libraries of "helper" functions used in snippets.
  These hinder the ability to copy+paste directly into Lean.
- Minimize Mathlib usage.
  - If mathlib is required, always add a notice in the chapter heading on
    how to set up mathlib.
  - Keep mathlib usage to the latter portion of the text after familiarity with 
    most baseline concepts.
- Prefer WET (Write Everything Twice) over DRY (Don't Repeat Yourself) when 
  given the choice.

