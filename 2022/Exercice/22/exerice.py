import os
import utils
import re
import doctest

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


def get_forward_square(pos,edges, map):
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
        return *forward_pos, pos[2]
    else:
        return edges[pos]

def navigate(map, edges, instructions):
    pos = (([x for x in range(len(scans[0])) if (x, 0) in map.keys()][0]), 0, 0)

    for curr_instruction in instructions:
        for i in range(curr_instruction[0]):
            coord_next_square = get_forward_square(pos, edges, map)
            if map[coord_next_square[:2]] == '#':
                break
            pos = coord_next_square

        #N pr la dernière instruction
        if curr_instruction[1] != 'N':
            new_rotation = (pos[2] + (1 if curr_instruction[1] == "R" else -1)) % 4
            pos = (pos[0], pos[1], new_rotation)

    return pos


def compute_edges(map):
    edges = {}
    # 7 transition spéciale a faire

    # 1 à 4
    x1 = 100
    y1 = 0
    x2 = 0
    y2 = 199
    for n in range(50):
            edges[(x1+n,y1, 3)] = (x2+n,y2,3)
            edges[(x2+n,y2, 1)] = (x1+n,y1,1)


    #1 à 5
    x1 = 149
    y1 = 0
    x2 = 99
    y2 = 149
    for n in range(50):
        edges[(x1, y1+n, 0)] = (x2, y2-n, 2)
        edges[(x2, y2-n, 0)] = (x1, y1+n, 2)



    # 1 à 3
    x1 = 100
    y1 = 49
    x2 = 99
    y2 = 50
    for n in range(50):
        edges[(x1 + n, y1, 1)] = (x2, y2 + n, 2)
        edges[(x2, y2 + n, 0)] = (x1 + n, y1, 3)


    # 2 à 4
    x1 = 50
    y1 = 0
    x2 = 0
    y2 = 150
    for n in range(50):
        edges[(x1 + n, y1, 3)] = (x2, y2 + n, 0)
        edges[(x2, y2 + n, 2)] = (x1 + n, y1, 1)

    # 2 à 6
    x1 = 50
    y1 = 0
    x2 = 0
    y2 = 149
    for n in range(50):
        edges[(x1, y1 + n, 2)] = (x2, y2 - n, 0)
        edges[(x2, y2 - n, 2)] = (x1, y1 + n, 0)

    # 3 à 6
    x1 = 50
    y1 = 99
    x2 = 49
    y2 = 100
    for n in range(50):
        edges[(x1, y1 - n, 2)] = (x2 - n, y2, 1)
        edges[(x2 - n, y2, 3)] = (x1, y1 - n, 0)

    # 4 à 5
    x1 = 50
    y1 = 149
    x2 = 49
    y2 = 150
    for n in range(50):
        edges[(x1 + n, y1, 1)] = (x2, y2 + n, 2)
        edges[(x2, y2 + n, 0)] = (x1 + n, y1, 3)

    #print la map
    txt = ""
    done_char = {}
    i = 0
    for y in range(200):
        for x in range(150):
            char = " "
            for orientation in range(4):
                if (x,y,orientation) in edges.keys():
                    if (x,y) in done_char:
                        char = done_char[(x,y)]
                    else:
                        char = str(i)
                        i+=1
                        done_char[edges[(x,y,orientation)][:2]] = char
            txt += char

        txt += "\n"
    #print(txt)


    return edges
if __name__ == "__main__":
    map, instructions = parse_input(scans)
    edges = compute_edges(map)
    pos = navigate(map,edges, instructions)
    print(str(1000 * (pos[1] + 1) + 4 * (pos[0] + 1) + pos[2]))
