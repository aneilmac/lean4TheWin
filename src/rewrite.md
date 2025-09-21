# Rewriting the Script

As discussed in the previous section, theorems are special compile-only
functions. Functions take arguments. What does a theorem that takes arguments
look like?

```lean
theorem nat_rfl (x : Nat) : x = x := rfl
```

If we think of this like a function, then `x` can be any expression that is a
`Nat`. So what this means is, for _all_ `Nat`s, `x = x`.

This also works for the following definition

```lean
theorem nat_rfl : ∀ (x : Nat), x = x := fun x ↦ rfl
```

Like the chapter on `def`, we can use `#print` to see what the "normal-form"
definition looks like.

~~~admonish example title=""
```lean
theorem nat_rfl (x : Nat) : x = x := rfl
#print nat_rfl
```
@[defeq] theorem nat_rfl : ∀ (x : Nat), x = x :=
fun x ↦ rfl
~~~

We seem some new syntax again. You can ignore `@[defeq]` and focus on the
function itself.

~~~admonish info
`@[defeq]` is an _attribute_ that has automatically being added to `nat_rfl`. Attributes are _metadata_ attached to functions/theorems. The user can add 
attributes themselves, or Lean may add them automatically. 

Just know that Whenever you see `@[...]` 
appear before a function/theorem, it is to provide extra information about 
the theorem and its usage.
Attributes are utilized by Lean's more advanced tactics/features.
~~~

What is important to focus on is `∀ (x : Nat)`, `∀` stands for "for all", 
and can be written as `∀`, `\forall` or `forall`. 

TODO: Losing the plot here. Skip/change.