import os
import utils
import queue

scans = utils.read_file(os.getcwd() + "\\input.txt")


def parse_stacks(scan):
    scan.reverse()
    stacks = [[],[],[],[],[],[],[],[],[]]
    for line in scan:
        args = [line[1 + i*4] for i in range(9)]
        for i, arg in enumerate(args):
            if arg != " ":
                stacks[i].append(arg)
    return stacks


def parse_instructions(scan):
    instructions = []
    for line in scan:
        a = int(line.split("move ")[1].split(" from")[0])
        b = int(line[-6]) - 1
        c = int(line[-1]) - 1
        instructions.append((a,b,c))
    return instructions


def compute_instructions(stacks, instructions):
    for a,b,c in instructions:
        for i in range(a):
            current = stacks[b].pop()
            stacks[c].append(current)
    return stacks


def compute_instructions2(stacks, instructions):
    for a, b, c in instructions:
        block_to_move = stacks[b][-a:].copy()
        [stacks[b].pop() for i in range(a)]
        stacks[c].extend(block_to_move)
    return stacks


if __name__ == "__main__":
    stacks = parse_stacks(scans[:8])
    instructions = parse_instructions(scans[10:])
    stacks = compute_instructions2(stacks, instructions)
    print("".join([stack.pop() for stack in stacks]))

