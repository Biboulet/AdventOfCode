import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def is_vertical(coord1, coord2):
    return coord1[0] == coord2[0]


def parse_input():
    map = {}

    for x in range(1000):
        for y in range(170):
            map[(x,y)] = 0

    for line in scans:
        args = line.split(" -> ")
        for i in range(len(args)-1):
            coord1 = [int(coord) for coord in args[i].split(",")]
            coord2 = [int(coord) for coord in args[i+1].split(",")]

            if is_vertical(coord1,coord2):
                for y in range(min([coord1[1], coord2[1]]), max([coord1[1], coord2[1]]) +1):
                    map[(coord1[0], y)] = 1
            else:
                for x in range(min([coord1[0], coord2[0]]), max([coord1[0], coord2[0]]) +1):
                    map[(x, coord2[1])] = 1

    maxY = max([coord[1] for coord in map.keys() if map[coord] == 1])
    for x in range(1000):
        map[(x,maxY+2)] = 1

    return map


def simulate_sand_drop(map):
    sand_pos = (500,0)
    sand_count = 0
    while True:

        #collision
        if map[(sand_pos[0],sand_pos[1]+1)] == 1:
            if map[(sand_pos[0]-1, sand_pos[1]+1)] == 0:
                map[sand_pos] = 0
                sand_pos = (sand_pos[0] - 1, sand_pos[1] + 1)
                map[sand_pos] = 1

            elif map[(sand_pos[0]+1, sand_pos[1]+1)] == 0:
                map[sand_pos] = 0
                sand_pos = (sand_pos[0] + 1, sand_pos[1] + 1)
                map[sand_pos] = 1

            else:
                if sand_pos == (500, 0):
                    return sand_count +1
                sand_pos = (500, 0)
                sand_count+=1
                print(sand_count)
                continue

        else:
            map[sand_pos] = 0
            sand_pos = (sand_pos[0], sand_pos[1] + 1)
            map[sand_pos] = 1



if __name__ == "__main__":
    map = parse_input()
    print(simulate_sand_drop(map))

