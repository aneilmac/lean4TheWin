

#eval List.reverse ["A", "B", "C"]

#check List.reverse

#eval List.append ["A"] ["B"]

#eval List.append (List.reverse ["C", "B", "A"]) (List.reverse ["F", "E", "D",])

#check List.append

#print List.append


#print List.append

#print List


#check List.append

#eval List.head! [1]

#eval List.length [1, 2, 3]

#eval List.range 100

def five : Int := 5
#eval [-five]

-- def double x := x * 2
-- #print double

-- def double' : Nat → Nat := fun x => x * x

--#check double

def add : Nat → (Nat → Nat) := fun x ↦ fun y ↦ x + y
#print add

#check True  -- (True : Prop)
#check False -- (False : Prop)
#check True = False -- (True = True : Prop)

#check (True : Bool)

#eval True

theorem five_eq_five : 5 = 5 := rfl

theorem example_two : 3 + 5 * 4 = 4 * 5 + 3 := rfl

theorem nat_rfl (x : Nat) : x = x := rfl
#print nat_rfl

theorem example_three : true && false == false && true := rfl


def double := fun x => x * 2

#eval 2^(double 32)


def add_and_double := fun x y => (x + y) * 2

#eval add_and_double 3 4

def sum_and_triple := fun x y z => (x + y + z) * 3

#eval sum_and_triple 3 (add_and_double 1 1) 5


def o'keeffe's := 42

def foo : List Int := [1, 2, 3]


def myFun (a b : Nat) : if a == b then String else Int :=
    match a == b with
    | true => "Hello"
    | false => (42 : Int)



#eval myFun 4 3

#print Nat

#check and

#check true == false

#check true ^^ false

def myList := [1, 2, 3]


#check List.range

#eval List.range' 1 100

#eval if true then true else false

#check Type

def idType' {α : Type} (x : α) := x

#check idType' (Type 2)

def id' {α} (x : α) := x

#check (∃ (x : Nat), 3*x < 3)
#check (∀ (x : Nat), 0 ≤ x)
#check 1 = 2

#check Function.const

--def const (a b : Nat) := a

--def const.{u} {α : Sort u} (a b : α) := a

--def const.{u} {α : Sort u} {β : Sort u} (a : α) (b : β) := a

def const {α β} (a : α) (b : β) := a

#eval const false Type
