import os
import utils
import re

scans = utils.read_file(os.getcwd() + "\\input.txt")
offset = 100

def parse_input(scans):
    map = {}

    for x in range(-offset, len(scans[0]) + offset):
        for y in range(-offset, len(scans) + offset):
            map[(x, y)] = '.'

    for y, line in enumerate(scans):
        for x, char in enumerate(line):
            map[(x, y)] = char

    return map


def get_empty_space_in_rect(map):
    minX = min([key[0] for key in map.keys() if map[key] == "#"])
    maxX = max([key[0] for key in map.keys() if map[key] == "#"])
    minY = min([key[1] for key in map.keys() if map[key] == "#"])
    maxY = max([key[1] for key in map.keys() if map[key] == "#"])

    count = 0
    for x in range(minX, maxX + 1):
        for y in range(minY, maxY + 1):
            if map[x, y] == ".":
                count += 1
    return count


def is_surrounded(key, map):
    if map[(key[0] + 1, key[1])] == "#":
        return True
    elif map[(key[0] - 1, key[1])] == "#":
        return True
    elif map[(key[0], key[1] + 1)] == "#":
        return True
    elif map[(key[0], key[1] - 1)] == "#":
        return True
    elif map[(key[0] + 1, key[1] + 1)] == "#":
        return True
    elif map[(key[0] + 1, key[1] - 1)] == "#":
        return True
    elif map[(key[0] - 1, key[1] + 1)] == "#":
        return True
    elif map[(key[0] - 1, key[1] - 1)] == "#":
        return True
    return False


def get_next_move(key, i, map):
    for _i in range(4):
        real_index = (_i + i) % 4

        # north
        if real_index == 0:
            if map[(key[0] - 1, key[1] - 1)] == "." and map[(key[0], key[1] - 1)] == "." and map[(key[0] + 1, key[1] - 1)] == ".":
                return key[0], key[1] - 1
        # south
        elif real_index == 1:
            if map[(key[0] - 1, key[1] + 1)] == "." and map[(key[0], key[1] + 1)] == "." and map[(key[0] + 1, key[1] + 1)] == ".":
                return key[0], key[1] + 1
        # west
        elif real_index == 2:
            if map[(key[0] - 1, key[1] - 1)] == "." and map[(key[0] - 1, key[1])] == "." and map[(key[0] - 1, key[1] + 1)] == ".":
                return key[0] - 1, key[1]

        # east
        elif real_index == 3:
            if map[(key[0] + 1, key[1] - 1)] == "." and map[(key[0] + 1, key[1])] == "." and map[(key[0] + 1, key[1] + 1)] == ".":
                return key[0] + 1, key[1]

    return key


def simulate_map(map, rounds):
    for i in range(rounds):
        print(i)
        # first half
        moves_instructions = {}  # moves_instructions[base_pos] = new_pos
        forbidden_pos = []
        for key, val in map.items():
            if val == ".":
                continue

            if is_surrounded(key, map):
                next_move = get_next_move(key, i, map)

                if next_move in moves_instructions.values():
                    forbidden_pos.append(next_move)

                moves_instructions[key] = next_move

        has_moved = False
        # second_half
        for old_pos, new_pos in moves_instructions.items():
            if new_pos in forbidden_pos:
                continue
            map[old_pos] = "."
            map[new_pos] = '#'
            has_moved = True

        if not has_moved:
            print(i+1)
            return map

    return map


def print_dict(dict):
    txt = ""
    minX = -offset
    maxX = offset + len(scans[0])
    minY = -offset
    maxY = offset + len(scans)
    for y in range(minY, maxY):
        for x in range(minX, maxX):
            txt += dict[(x, y)]
        txt += "\n"
    print(txt)



if __name__ == "__main__":
    map = parse_input(scans)
    map = simulate_map(map, 1000)
    print(get_empty_space_in_rect(map))
