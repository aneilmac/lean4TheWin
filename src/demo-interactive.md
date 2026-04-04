# Interactive demo


```lean
theorem even_nth {i : Nat} : 3 ∣ i ↔ Even (nth (i + 2)) := by
  --## TODO
  apply Iff.intro
  . intro h
    rw [dvd_iff_exists_eq_mul_left] at h
    have ⟨c, hc⟩ := h; rw [hc]
    apply even_for_3_mul
  . intro h;
    contrapose h
    apply Nat.emod_pos_of_not_dvd at h
    apply exists_ne_dvd_3 at h
    cases h
    case inl h =>
      have ⟨c, ch⟩ := h
      simp [ch]; apply odd_for_3_mul_add_1
    case inr h =>
      have ⟨c, ch⟩ := h
      simp [ch]; apply odd_for_3_mul_add_2
```

```lean-interactive
theorem even_nth {i : Nat} : 3 ∣ i ↔ Even (nth (i + 2)) := by
  --## TODO
  apply Iff.intro
  --## TODO 2
  . intro h
    rw [dvd_iff_exists_eq_mul_left] at h
    have ⟨c, hc⟩ := h; rw [hc]
    apply even_for_3_mul
  . intro h;
    --## LINE 1
    --## LINE 2
    contrapose h
    apply Nat.emod_pos_of_not_dvd at h
    apply exists_ne_dvd_3 at h
    cases h
    case inl h =>
      have ⟨c, ch⟩ := h
      simp [ch]; apply odd_for_3_mul_add_1
    case inr h =>
      --## TODO 3
      have ⟨c, ch⟩ := h
      simp [ch]; apply odd_for_3_mul_add_2
```