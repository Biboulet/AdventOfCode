import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def find_error(line):
    assert len(line) %2==0
    first_part_char = line[:len(line)//2]
    second_part_char = line[len(line)//2:]

    for char in first_part_char:
        if char in second_part_char:
            return char


def solve(scans):
    score = 0
    for line in scans:
        a = find_error(line)
        score += ord(a) - 64 + 26 if str.isupper(a) else ord(a) - 96
    return score


def find_commun(param):
    return set(param[0]).intersection(set(param[1])).intersection(set(param[2])).pop()


def solve2(scans):
    score = 0
    for i in range(0, len(scans), 3):
        a = find_commun(scans[i:i+3])
        score += ord(a) - 64 + 26 if str.isupper(a) else ord(a) - 96
    return score


if __name__ == "__main__":
    print(solve2(scans))
