import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def get_starting_index(line):
    three_last_char = line[:4]
    for i, char in enumerate(line[3:]):
        three_last_char = three_last_char[1:] + char
        if all([three_last_char.count(char) == 1 for char in three_last_char]):
            return i + 4


def get_starting_index2(line):
    three_last_char = line[:14]
    for i, char in enumerate(line[14:]):
        three_last_char = three_last_char[1:] + char
        if all([three_last_char.count(char) == 1 for char in three_last_char]):
            return i + 15


if __name__ == "__main__":
    print(get_starting_index(scans[0]))
    print(get_starting_index2(scans[0]))
