import os
import utils
import re

scans = utils.read_file(os.getcwd() + "\\input.txt")


def solve(decrypted_list):
    # 5 ; 4333
    index_of_0 = decrypted_list.index(0)
    return sum([decrypted_list[(index_of_0 + 1000) % len(decrypted_list)],
                decrypted_list[(index_of_0 + 2000) % len(decrypted_list)],
                decrypted_list[(index_of_0 + 3000) % len(decrypted_list)]])


def parse_input(scans):
    list = []
    for i, line in enumerate(scans):
        list.append(int(line)*811589153)
    return list


def move_object(decrypted_list, old_index, new_index):
    val = decrypted_list.pop(old_index)

    if new_index <= old_index:
        decrypted_list.insert(new_index, val)
    else:
        decrypted_list.insert(new_index - 1, val)


def get_index_in_decrypted_list(decrypted_list, index_in_crypted):
    for i, curr_val in enumerate(decrypted_list):
        if curr_val[1] == index_in_crypted:
            return i


def decrypt_list(crypted_list):
    # (val, old_index)
    decrypted_list = [(val, i) for i, val in enumerate(crypted_list)]

    for i in range(10):
        for i, val in enumerate(crypted_list):
            current_index = get_index_in_decrypted_list(decrypted_list, i)

            new_index = ((current_index + (val % (len(crypted_list) - 1)))%len(crypted_list))+1

            move_object(decrypted_list, current_index, new_index)

    return [v[0] for v in decrypted_list]


if __name__ == "__main__":
    list = parse_input(scans)
    list = decrypt_list(list)
    print(solve(list))
