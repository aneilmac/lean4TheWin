

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
