import os
import utils
import re

scans = utils.read_file(os.getcwd() + "\\input.txt")


def parse_input(scans):
    map = {}
    for y, line in enumerate(scans):
        for x, char in enumerate(line):
            map[(x, y)] = char if char == "#" else ("" if char == "." else char)
    return map


def simulate_map(map):
    all_maps = [map.copy()]
    for i in range(1000):

        current_map = all_maps[i].copy()
        for key, val_list in current_map.copy().items():
            if val_list == "#" or val_list == "":
                continue

            for val in val_list:
                current_map[key] = current_map[key].replace(val, "", 1)

                if val == "^":
                    if not key[1] == 1:
                        current_map[(key[0], key[1] - 1)] += val
                    else:
                        current_map[(key[0], len(scans) - 2)] += val

                elif val == "v":
                    if not key[1] == len(scans) - 2:
                        current_map[(key[0], key[1] + 1)] += val
                    else:
                        current_map[(key[0], 1)] += val

                elif val == "<":
                    if not key[0] == 1:
                        current_map[(key[0] - 1, key[1])] += val
                    else:
                        current_map[(len(scans[0]) - 2, key[1])] += val

                elif val == ">":
                    if not key[0] == len(scans[0]) - 2:
                        current_map[(key[0] + 1, key[1])] += val
                    else:
                        current_map[(1, key[1])] += val

        all_maps.append(current_map)
    return all_maps


def get_first_situations(available_situations, min_target):
    print(min_target)
    for i in range(2):
        for situation in available_situations:
            if situation[1] == min_target:
                return situation
        min_target += 1
    print("error")


def get_available_moves(map_for_minutes, current_situations):
    pos = current_situations[0]
    min = current_situations[1]

    available_next_moves = []

    # wait
    if map_for_minutes[min + 1][pos] == "":
        available_next_moves.append(pos)
    # haut
    if map_for_minutes[min + 1][(pos[0], pos[1] - 1)] == "":
        available_next_moves.append((pos[0], pos[1] - 1))
    # bas
    if map_for_minutes[min + 1][(pos[0], pos[1] + 1)] == "":
        available_next_moves.append((pos[0], pos[1] + 1))
    # gauche
    if map_for_minutes[min + 1][(pos[0] - 1, pos[1])] == "":
        available_next_moves.append((pos[0] - 1, pos[1]))
    # droite
    if map_for_minutes[min + 1][(pos[0] + 1, pos[1])] == "":
        available_next_moves.append((pos[0] + 1, pos[1]))

    return [(_pos, min + 1) for _pos in available_next_moves]


def find_shortest_way(map_for_minutes):
    # dijkstra

    target = (len(scans[0]) - 2, len(scans) - 1)
    available_situations = [((1, 1), 1)]  # (pos, minutes)

    last_min = 1
    while len(available_situations) > 0:
        current_situations = get_first_situations(available_situations, last_min)
        available_situations.remove(current_situations)
        last_min = current_situations[1]

        if current_situations[0] == target:
            return current_situations[1]

        if current_situations[0][1] == 0:
            continue

        available_next_moves = get_available_moves(map_for_minutes, current_situations)
        available_situations.extend(available_next_moves)


def print_dict(dict):
    txt = ""
    minX = 0
    maxX = len(scans[0])
    minY = 0
    maxY = len(scans)
    for y in range(minY, maxY):
        for x in range(minX, maxX):
            txt += "." if dict[(x, y)] == "" else (
                "#" if dict[(x, y)] == "#" else (str(len(dict[(x, y)])) if len(dict[(x, y)]) > 1 else dict[(x, y)]))
        txt += "\n"
    print(txt)


if __name__ == "__main__":
    map = parse_input(scans)
    map_for_minutes = simulate_map(map)
    print("done simulating")
    print(find_shortest_way(map_for_minutes))
