import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def parse_input(scans):
    pairs = []
    for line in scans:
        elfs = line.split(",")
        elf1 = elfs[0].split("-")
        elf2 = elfs[1].split("-")
        pairs.append([(int(elf1[0]), int(elf1[1])), (int(elf2[0]), int(elf2[1]))])

    return pairs


def get_num_of_fully_contained_pairs(pairs):
    count = 0
    for pair in pairs:
        if (pair[0][0] <= pair[1][0] and pair[1][1] <= pair[0][1]) or (pair[1][0] <= pair[0][0] and pair[0][1] <= pair[1][1]):
            count +=1
    return count


def contains(a, b):
    if b[0] <= a[0] <= b[1] or b[0] <= a[1] <= b[1]:
        return True
    return False


def get_num_of_overlaping(pairs):
    count = 0
    for pair in pairs:
        if contains(pair[0], pair[1]) or contains(pair[1], pair[0]):
            count += 1
    return count


if __name__ == "__main__":
    pairs = parse_input(scans)
    print(get_num_of_fully_contained_pairs(pairs))
    print(get_num_of_overlaping(pairs))
