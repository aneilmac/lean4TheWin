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

### Chapter 0

1. Student knows Lean is a programming language.
1. Student knows Lean is proof verifier.

### Chapter 1

1. Student can install vscode
1. Student can install lean extension in vscode.
1. Student can create a lean file in vscode. 
1. Student can call the `#print` statement
1. Student understands and can interpret lean infoview.

### Chapter 2

1. Student understands comments
1. Students understands simple numeric expressions
1. Student understands `#eval`
1. Student learns of `Nat` type.
1. Student understands how to write custom symbols.
   - Is given appendix table of common symbols in course.
     `\forall`, `\exists` `\<-`, `\.`, `alpha`, `\beta`
1. Student knows to hover over symbols for more details

### Chapter 3

1. Student understands `def` keyword in relation to declaration.
1. Student understands basic type-specifier syntax (e.g `: Nat`.)
1. Student understands writing simple functions.
1. Student understands `#check`
1. Student introduced to `List Nat`, `[]`, `++`, and `List.reverse`
1. Student understands they can call `[1, 2].reverse`, and
   `List.reverse [1, 2]` are the same call with different syntax.
1. Student understands lean file is executed in order.

### Chapter 4

1. Student understands lambda types (`fun` kewyord)
1. Student understands partial function applications / currying.
1. Student understands shorthand for type parameters.
1. Student understands types can be omitted if compiler can guess.
1. Student understands functions calling functions
1. Student understands `Bool` type. (`==`, `not`, `if _ then _ else`)
1. Student understands `let` in `def`

### Chapter 5

1. Student understands a simple theorem `theorem x : 2 = 2 := rfl`
1. Student understands `noncomputable`
1. Student understands cliff-notes of Curry-Howard isomorphism
   (a definition to a function is equivalent to a proof).
1. student understands `example` keyword
1. Student understands reflexivity and the `refl` keyword.
1. Student understands `=` is a form of equality different 
   from `==`.
1. Student understands there is a `Prop` type.
1. Student understands `Prop`s don't need to be true statements.

### Chapter 6

1. Student leans of the `by` keyword.
1. Student understands `\forall` and equivelence to parameter.
1. Student understands `rewrite` tactic.
1. Student understands the `rw` tactic.
1. Student understands the `rfl` tactic.
1. Student understands `[rw \<-]`
1. Student understands multiple (`rw [a, b]`) in one statement.
1. Student comfortable with proving some peano axioms.
1. Student understands using `#loogle`.
1. Student understands `sorry`

### Chapter 7

1. Student understands `struct` definitions
1. Student understands tuples.
1. Student understands `String` type.
1. Student understands `Fin` type.
1. Student understands constructors (and making structs)
1. Student understands namespaces in a type context.
1. Student understands struct brace style construction
1. Student understands member accessing
1. Student understands updating struct

### Chapter 8

1. Student understands `And` struct and `\and`.
1. Student understands `Or` struct and `\or`.
1. Student understands `Iff` (.mp and .mpr)
1. Student understands `_` wildcard.
1. Student understands `intro`, `rintro`
1. Student understands destructuring using `\<`, `\>`
1. Student understands `rw at`
1. Student understands `simp`, `simp [x]`, `simp only` and `simp`
1. Student understands `apply`, `apply at` (backwards and forwards).
1. Student understands `exact` and `exact?`
1. Student understands `symm`
1. Student understands `rcases` for destructuring
1. Student understands contrapositive (`mt`).
1. Student understands selecting goals `\.`
1. Student understands switching goals.

### Chapter 9

1. Student understands implict parameters use `{}`
1. Student understands headlines of Type universes.
1. Student understands link between `Sort (n + 1)` and `Type n`
1. Student now understands parameters of `#check List`
1. Student understands parallel to generics.
1. Student understands `@` syntax.
1. Student understands typeclass syntax `[]`
1. Student understands applying typeclasses
2. Student understands common typeclasses.
1. Student understands creating typeclasses (TODO: May need to be separate chapter)

### Chapter 10

1. Student understands inductive types
1. Student understands inductive type constructors and ellision
1. Student understands match statement
1. Student understands `Nat` is an inductive type
1. Student understands `List` is an inductive type.
1. Student understands `induction`
1. Student understands `induction generalizing`
1. Student understandings `induction using`
1. Student understands `rcases` for goals
1. Student understands inductive type proofs may be required

## Chapter 11 

1. Student understands there is a difference between constructivist and classical logic.
1. Student understands that there is an opt-in classical mode.
1. Student understands the 5 classical theorems.
1. Student understands relation to halting problem.
1. Student knows `by_cases` tactic
1. Student knows `by_contra` tactic
1. Student knows `exfalso` tactic
1. Student knows about the `Exists` struct
1. Student knows about `exists` tactic.

### Chapter 12 

1. Student knows about mathlib
1. Student knows how to install mathlib.
1. Student knows how to use `#loggle`
1. Student knows `ring` tactic
1. Student knows `omega` tactic
1. Student knows about `Ring` class
1. Student knows about `Set` and set syntax.

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

