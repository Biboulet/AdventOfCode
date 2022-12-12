import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def compute_sytem(scans):
    cycles_instructions = []
    for line in scans:
        args = line.split()
        if args[0] == "addx":
            cycles_instructions.append(0)
            cycles_instructions.append(int(args[1]))
        elif args[0] == "noop":
            cycles_instructions.append(0)

    values_of_X_at_end_of_cycle = [None] * (len(cycles_instructions)+1)
    X = 1
    for i in range(len(cycles_instructions)):
        X += cycles_instructions[i]
        values_of_X_at_end_of_cycle[i+1] = X

    return values_of_X_at_end_of_cycle


def get_strengths(list, X_Values):
    final = []
    for i in list:
        final.append(X_Values[i]*i)
    return final

if __name__ == "__main__":
    X_Values = compute_sytem(scans)
    a = get_strengths([20,60,100,140,180,220], X_Values)
    print(sum(a))
    a=0