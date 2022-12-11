import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


class File:
    def __init__(self, v, n):
        self.value = v
        self.name = n


class Folder:
    def __init__(self, n, f):
        self.subs = {}
        self.total = 0
        self.name = n
        self.father = f

    def compute_total_value(self):
        total = 0
        for sub in self.subs.values():
            total += sub.value if type(sub) is File else sub.compute_total_value()
        self.total = total
        return total


def add_sub_to(directory, args):
    if args[0] == "dir":
        new_dir = Folder(args[1], directory)
        directory.subs[args[1]] = new_dir
    else:
        new_file = File(int(args[0]), args[1])
        directory.subs[args[0]] = new_file


def parse_hierarchie(scans):
    main = Folder("/", None)
    current_directory = main
    for line in scans:

        if line[0] == "$":
            command = line[2:4]
            if command == "cd":
                arg = line.split()[2]
                if arg == "..":
                    current_directory = current_directory.father
                    continue
                elif arg == "/":
                    current_directory = main
                    continue
                else:
                    current_directory = current_directory.subs[arg]
                    continue
        else:
            args = line.split()
            add_sub_to(current_directory, args)
    return main


def file_lower_than(num, folder, list):
    if folder.compute_total_value() <= num:
        list.append(folder)

    for sub in folder.subs.values():
        if type(sub) is Folder:
            file_lower_than(num, sub, list)
    return list

def file_greater_than(num, folder, list):
    if folder.compute_total_value() >= num:
        list.append(folder)

    for sub in folder.subs.values():
        if type(sub) is Folder:
            file_greater_than(num, sub, list)
    return list


if __name__ == "__main__":
    main = parse_hierarchie(scans)
    main.compute_total_value()
    minimum_space_that_needs_to_be_deleted = 30000000-(70000000-main.total)
    greater_than = file_greater_than(minimum_space_that_needs_to_be_deleted, main, [])
    print(min([file.total for file in greater_than]))
