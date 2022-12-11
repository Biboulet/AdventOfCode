import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def move_head(head, dir):
    if dir == "U":
        return head[0], head[1] + 1
    if dir == "D":
        return head[0], head[1] - 1
    if dir == "L":
        return head[0] - 1, head[1]
    if dir == "R":
        return head[0] + 1, head[1]


def is_adjacent(tail, head):
    return abs(tail[0] - head[0]) <= 1 and abs(tail[1] - head[1]) <= 1


def move_tail(tail, head):
    if is_adjacent(tail, head):
        return tail

    a, b = 0, 0
    if head[0] < tail[0]:
        a = -1
    elif head[0] > tail[0]:
        a = 1
    if head[1] < tail[1]:
        b = -1
    elif head[1] > tail[1]:
        b = 1
    return tail[0] + a, tail[1] + b


def sovle(scans):
    knots = [(0, 0) for i in range(10)]
    place_visited = set()

    for line in scans:
        args = line.split()
        for i in range(int(args[1])):
            knots[0] = move_head(knots[0], args[0])
            for i, item in enumerate(knots):
                if i == 9:
                    break
                knots[i + 1] = move_tail(knots[i + 1], knots[i])
                if i == 8:
                    place_visited.add(knots[i + 1])

    return len(place_visited)


if __name__ == "__main__":
    print(sovle(scans))
