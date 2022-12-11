import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def parse_input(scans):
    map = {}
    for y, line in enumerate(scans):
        for x, val in enumerate(line):
            map[(x, y)] = int(val)
    return map


def get_visible_trees(map):
    visible_trees = set()
    # gauche et droite
    for y in range(len(scans)):
        # gauche
        previous_num = -1
        for x in range(len(scans[0])):
            if map[(x, y)] > previous_num:
                visible_trees.add((x, y))
                previous_num = map[(x, y)]

        # droite
        previous_num = -1
        for x in range(len(scans[0]) - 1, 0, -1):
            if map[(x, y)] > previous_num:
                visible_trees.add((x, y))
                previous_num = map[(x, y)]
    # haut et bas
    for x in range(len(scans[0])):
        # haut
        previous_num = -1
        for y in range(len(scans)):
            if map[(x, y)] > previous_num:
                visible_trees.add((x, y))
                previous_num = map[(x, y)]

        # bas
        previous_num = -1
        for y in range(len(scans) - 1, 0, -1):
            if map[(x, y)] > previous_num:
                visible_trees.add((x, y))
                previous_num = map[(x, y)]
    return visible_trees


def get_scenic_score(map, key):
    scenic_score = 1
    val = map[key]
    X, Y = key

    # gauche
    vision_score = 0
    for x in range(X-1,-1,-1):
        vision_score += 1
        if map[(x, Y)] >= val:
            break
    scenic_score *= vision_score

    # droite
    vision_score = 0
    for x in range(X+1, len(scans[0])):
        vision_score += 1
        if map[(x, Y)] >= val:
            break
    scenic_score *= vision_score

    # haut
    vision_score = 0
    for y in range(Y-1, -1, -1):
        vision_score += 1
        if map[(X, y)] >= val:
            break
    scenic_score *= vision_score

    # bas
    vision_score = 0
    for y in range(Y + 1, len(scans)):
        vision_score += 1
        if map[(X, y)] >= val:
            break
    scenic_score *= vision_score
    return scenic_score


def get_all_scenic_scores(map):
    scenic_scores = []
    for key in map.keys():
        scenic_scores.append(get_scenic_score(map, key))
    return scenic_scores


if __name__ == "__main__":
    map = parse_input(scans)
    print(max(get_all_scenic_scores(map)))
