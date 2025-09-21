

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

def double x := x * 2
#print double

def double' : Nat → Nat := fun x => x * x

#check double

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
