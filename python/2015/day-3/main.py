from collections import defaultdict

def read_input(file="/Users/nshaw/Code/kolasniwash/advent-of-code/python/2015/day-3/input.txt"):
    with open(file, "r") as f:
        return f.read()


def update_loc(loc, step):
    match step:
        case "^":
            return (loc[0], loc[1] + 1)
        case "v":
            return (loc[0], loc[1] - 1)
        case ">":
            return (loc[0] + 1, loc[1])
        case "<":
            return (loc[0] - 1, loc[1])


def count_locations(steps, num_santas=1):

    start_loc = (0,0)
    current_santa_locs = {k:start_loc for k in range(num_santas)}
    unique_locs = dict()

    index = 0
    while index < len(steps):
        for santa in range(num_santas):
            step = steps[index]
            loc = current_santa_locs[santa]
            cur_santa_loc = update_loc(loc, step)
            current_santa_locs[santa] = cur_santa_loc

            if cur_santa_loc not in unique_locs:
                unique_locs[cur_santa_loc] = 1

            index += 1

    return len(unique_locs)

if __name__ == "__main__":

    #2572
    #2631
    steps = read_input()
    total_stops = count_locations(steps, num_santas=1)
    print("Total stops 1 Santa: ", total_stops)
    total_stops = count_locations(steps, num_santas=2)
    print("Total stops 2 Santas: ", total_stops)
