

def three : Int := 3

#check three

def two : Int := 2

def myList : List Int := [1, two, 3, 4, 5]

#eval myList


def double x := x * 2

#check double

#print double


def add_three_and_double := fun x y z => (x + y + z) * 2
#check add_three_and_double
