
-- ANCHOR: simple_eval
#eval 3 + 4
-- ANCHOR_END: simple_eval

#eval "Hello World"

#eval 4 ^ 2

#eval 18 / 3

#eval 1 + 3 * 4 - 2
/-
-/
#eval (1 + 2) * 4 / 2.5

#eval 3 - 3

#eval "Hello" ++ 4

#eval 3 + "Hello"

#eval (4 : ℕ)

#eval 2^64 + 1

#eval (1 : Int) - (3 : Int)

#eval (2^64 + 1) * -1


#eval "Hello WØℝLΔ"

#eval "Hello " ++ "Bob! " ++ "H" ++ "o" ++ "w" ++ " Are you?"
