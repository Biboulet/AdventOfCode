import os
import utils
import re
import math

scans = utils.read_file(os.getcwd() + "\\input.txt")
PPMC = math.lcm(len(scans[0]) - 2, len(scans) - 2)


def parse_input(scans):
    map = {}
    for y, line in enumerate(scans):
        for x, char in enumerate(line):
            map[(x, y)] = char if char == "#" else ("" if char == "." else char)
    return map


def simulate_map(map):
    all_maps = [map.copy()]
    for i in range(PPMC+2):

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


def get_first_situations(available_situations, distance, min_target):
    for i in range(2):
        for situation in available_situations:
            if distance[situation] == min_target:
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
    if pos[1] != 0 and map_for_minutes[min + 1][(pos[0], pos[1] - 1)] == "":
        available_next_moves.append((pos[0], pos[1] - 1))
    # bas
    if pos[1] != len(scans)-1 and map_for_minutes[min + 1][(pos[0], pos[1] + 1)] == "":
        available_next_moves.append((pos[0], pos[1] + 1))
    # gauche
    if map_for_minutes[min + 1][(pos[0] - 1, pos[1])] == "":
        available_next_moves.append((pos[0] - 1, pos[1]))
    # droite
    if map_for_minutes[min + 1][(pos[0] + 1, pos[1])] == "":
        available_next_moves.append((pos[0] + 1, pos[1]))

    return [(_pos, (min + 1) % PPMC) for _pos in available_next_moves]


def find_shortest_way(map_for_minutes):
    # dijkstra

    start = ((1, 0), 0)
    total = 0

    for i in range(3):
        target = (len(scans[0]) - 2, len(scans) - 1) if i%2 == 0 else (1,0)
        distance = {((x, y), state): 9999 for state in range(PPMC) for x in range(1, len(scans[0]) - 1) for y in
                    range(len(scans))}
        available_situations = {start}

        distance[start] = 0
        lowest_time = 0

        while available_situations:
            current_situations = get_first_situations(available_situations, distance, lowest_time)
            available_situations.remove(current_situations)
            lowest_time = distance[current_situations]

            if current_situations[0] == target:
                print(distance[current_situations])
                total += distance[current_situations]
                start = current_situations
                break

            available_next_moves = get_available_moves(map_for_minutes, current_situations)
            for next_move in available_next_moves:
                distance[next_move] = lowest_time + 1
                available_situations.add(next_move)

    return total
if __name__ == "__main__":
    map = parse_input(scans)
    map_for_minutes = simulate_map(map)
    print("done simulating")
    print(find_shortest_way(map_for_minutes))
