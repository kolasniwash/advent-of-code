def read_input(file="input.txt"):
    with open(file, "r") as f:
        return f.readlines()

def wrapping_paper(lines):
    total = 0
    for line in lines:
        dims = line.strip("\n").split("x")
        l, w, h = int(dims[0]), int(dims[1]), int(dims[2])
        a1 = 2*l*w
        a2 = 2*w*h
        a3 = 2*h*l
        min_dim = min([a1, a2, a3])
        sum_dims = a1 + a2 + a3 + min_dim/2
        total += sum_dims

    return total

def ribbon(lines):
    total = 0
    for line in lines:
        dims = line.strip("\n").split("x")
        l, w, h = int(dims[0]), int(dims[1]), int(dims[2])
        p1, p2 = sorted([l, w, h])[:2]
        ribbon = p1 + p1 + p2 + p2 + l * w * h
        total += ribbon
    return total

if __name__ == "__main__":
    lines = read_input()
    total_paper = wrapping_paper(lines)
    print(total_paper)
    total_ribbon = ribbon(lines)
    print(total_ribbon)
