

def myList : List Int64 := [1, 2, 3]

#check String
#print String

#eval "hello " ++ "world"

#eval "h" ++ "ello"

#eval String.mk (List.reverse "ello".toList)

#check Fin

#eval myList
#check myList
#print myList

#eval myList.reverse

#eval List.reverse myList

#eval 5 :: 3 :: myList

#eval myList ++ [5, 4, 6]

def emptyList : List Int64 := []

theorem myList_eq_myList : myList = myList := rfl

inductive MyList where
  | nil
  | cons (head : Int64) (tail : MyList) : MyList


-- ANCHOR: my_theory
theorem my_theory {α β γ : Prop} (h : α -> β) (g: β -> γ) : (α -> γ) := by
  --## α β γ : Prop
  --## h : α → β
  --## g : β → γ
  --## ⊢ α → γ
  intro a
  --## α β γ : Prop
  --## h : α → β
  --## g : β → γ
  --## a : α
  --## ⊢ γ
  apply g
  --## α β γ : Prop
  --## h : α → β
  --## g : β → γ
  --## a : α
  --## ⊢ β
  apply h
    --## α β γ : Prop
  --## h : α → β
  --## g : β → γ
  --## a : α
  --## ⊢ α
  exact a
  --## No goals
-- ANCHOR_END: my_theory


def a := 15

def b := 6

def c := a * b

def q := "Hello World"

#eval q
