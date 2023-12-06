def read_input(file="input.txt"):
    with open(file, "r") as f:
        return f.read()

def floor_up_down(char: str) -> int:
    if char == "(":
        return 1
    elif char == ")":
        return -1
    return 0


def floor_count(input: str):
    count = 0
    for char in input:
        count += floor_up_down(char)
    return count

def basement_index(input: str):
    count = 0
    idx = 0
    for char in input:
        count += floor_up_down(char)
        idx += 1
        if count < 0:
            return idx
    return None

if __name__ == "__main__":
    input = read_input()
    floor = floor_count(input)
    print(floor)

    first_basement = basement_index(input)
    print(first_basement)