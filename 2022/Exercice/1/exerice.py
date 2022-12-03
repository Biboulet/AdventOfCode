import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def parse_input(scans):
    elves = []
    current_elf = []
    for line in scans:
        if line == "":
            elves.append(current_elf.copy())
            current_elf.clear()
            continue
        current_elf.append(int(line))
    return elves


def solvep2(elves_supplies):
    return sum([sum(val) for val in sorted(elves_supplies, key=lambda current:sum(current), reverse=True)[:3]])


if __name__ == "__main__":
    elves_supplies = parse_input(scans)
    print(solvep2(elves_supplies))
