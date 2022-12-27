import os
import utils
import re

scans = utils.read_file(os.getcwd() + "\\input.txt")


def parse_input(scans):
    map = {}
    for y, line in enumerate(scans[:-2]):
        for x, char in enumerate(line):
            if char != " ":
                map[x, y] = char

    instructions = []
    current_num = ""
    for char in scans[-1]:
        if char.isdigit():
            current_num += char
        else:
            instructions.append((int(current_num), char))
            current_num = ""
    instructions.append((int(current_num), "N"))

    return map, instructions


def get_opposite_square(pos, map):
    if pos[2] == 0:
        for x in range(len(scans[pos[1]])):
            if (x, pos[1]) in map.keys():
                return x, pos[1]

    elif pos[2] == 1:
        for y in range(len(scans) - 2):
            if (pos[0], y) in map.keys():
                return pos[0], y

    elif pos[2] == 2:
        for x in range(len(scans[pos[1]]) - 1, -1, -1):
            if (x, pos[1]) in map.keys():
                return x, pos[1]

    elif pos[2] == 3:
        for y in range(len(scans) - 3, -1, -1):
            if (pos[0], y) in map.keys():
                return pos[0], y

    print("error")


def get_forward_square(pos, map):
    forward_pos = (0, 0)
    if pos[2] == 0:
        forward_pos = (pos[0] + 1, pos[1])
    elif pos[2] == 1:
        forward_pos = (pos[0], pos[1] + 1)
    elif pos[2] == 2:
        forward_pos = (pos[0] - 1, pos[1])
    elif pos[2] == 3:
        forward_pos = (pos[0], pos[1] - 1)

    if forward_pos in map.keys():
        return forward_pos
    return get_opposite_square(pos, map)


def navigate(map, instructions):
    pos = (([x for x in range(len(scans[0])) if (x, 0) in map.keys()][0]), 0, 0)

    for curr_instruction in instructions:
        for i in range(curr_instruction[0]):
            coord_next_square = get_forward_square(pos, map)
            if map[coord_next_square] == '#':
                break
            pos = [coord_next_square[0], coord_next_square[1], pos[2]]

        if curr_instruction[1] != 'N':
            new_rotation = (pos[2] + (1 if curr_instruction[1] == "R" else -1)) % 4
            pos = (pos[0], pos[1], new_rotation)

    return pos


if __name__ == "__main__":
    map, instructions = parse_input(scans)
    pos = navigate(map, instructions)
    print(str(1000 * (pos[1] + 1) + 4 * (pos[0] + 1) + pos[2]))
