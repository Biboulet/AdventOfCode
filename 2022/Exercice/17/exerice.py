import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")

hitboxs = [[(0, 0), (1, 0), (2, 0), (3, 0)], [(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
           [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], [(0, 0), (0, 1), (0, 2), (0, 3)], [(0, 0), (0, 1), (1, 0), (1, 1)]]


def can_move_left(point, map):
    return map[(point[0] - 1, point[1])] == 0


def can_move_right(point, map):
    return map[(point[0] + 1, point[1])] == 0


def can_move_down(point, map):
    return map[(point[0], point[1] - 1)] == 0


def get_highest_rock_y(map, start):
    for y in range(start, start + 1000):
        raw_is_rocked = False
        for x in range(1, 8):
            raw_is_rocked = raw_is_rocked or map[x, y] == 1
        if not raw_is_rocked:
            return y - 1
    print("error")


# début + pattern répété + fin
def solve(scans):
    target_rock_count = 1000000000000
    map = {}
    for x in range(9):
        for y in range(10 ** 5):
            if y == 0:
                map[x, y] = 1
            elif x == 0 or x == 8:
                map[x, y] = 1
            else:
                map[x, y] = 0

    height_after_1_cycle = 0
    rock_after_1_cycle = 0

    repeted_height = 0
    repeted_rock = 0

    current_rock_pos = (3, 4)
    current_rock_type = 0

    rock_stopped_count = 0
    total_height = 0

    last_max_height = 0
    last_rock_count = 0

    for i in range(4):
        for char in scans[0]:
            # gaz
            if char == "<":
                if all([can_move_left(add_tuple(point, current_rock_pos), map) for point in
                        hitboxs[current_rock_type]]):
                    current_rock_pos = (current_rock_pos[0] - 1, current_rock_pos[1])
            else:
                if all([can_move_right(add_tuple(point, current_rock_pos), map) for point in
                        hitboxs[current_rock_type]]):
                    current_rock_pos = (current_rock_pos[0] + 1, current_rock_pos[1])

            # gravity
            if all([can_move_down(add_tuple(point, current_rock_pos), map) for point in hitboxs[current_rock_type]]):
                current_rock_pos = (current_rock_pos[0], current_rock_pos[1] - 1)
            else:
                # rest
                for pos in hitboxs[current_rock_type]:
                    map[add_tuple(pos, current_rock_pos)] = 1

                if rock_stopped_count == target_rock_count:
                    return get_highest_rock_y(map, current_rock_pos[1]) - last_max_height + total_height
                rock_stopped_count += 1
                current_rock_type = (current_rock_type + 1) % len(hitboxs)
                current_rock_pos = (3, get_highest_rock_y(map, current_rock_pos[1]) + 4)

        if i == 0:
            height_after_1_cycle = get_highest_rock_y(map, current_rock_pos[1] - 10)
            rock_after_1_cycle = rock_stopped_count

        elif i == 1:
            repeted_height = get_highest_rock_y(map, current_rock_pos[1] - 10) - height_after_1_cycle
            repeted_rock = rock_stopped_count - rock_after_1_cycle

            last_max_height = get_highest_rock_y(map, current_rock_pos[1] - 10)
            last_rock_count = rock_stopped_count

        elif i == 2:
            a = rock_stopped_count - last_rock_count
            b = get_highest_rock_y(map, current_rock_pos[1] - 10) - last_max_height
            assert a == repeted_rock
            assert b == repeted_height

            num_of_patterns = (target_rock_count - rock_after_1_cycle) // repeted_rock
            total_height = height_after_1_cycle + num_of_patterns * repeted_height - 1
            rock_stopped_count = rock_after_1_cycle + num_of_patterns * repeted_rock

            last_max_height = get_highest_rock_y(map, current_rock_pos[1] - 10)

        elif i == 2:
            print("error")


def print_dict(dict):
    txt = ""
    for y in range(3000, -1, -1):
        for x in range(9):
            txt += "#" if dict[(x, y)] == 1 else "."
        txt += "\n"
    return print(txt)


def add_tuple(t1, t2):
    return t1[0] + t2[0], t1[1] + t2[1]


if __name__ == "__main__":
    print(solve(scans))
