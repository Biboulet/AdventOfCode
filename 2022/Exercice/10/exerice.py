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

    values_of_X_at_end_of_cycle = [0] * (len(cycles_instructions)+1)
    X = 1
    for i in range(len(cycles_instructions)):
        X += cycles_instructions[i]
        values_of_X_at_end_of_cycle[i] = X

    return values_of_X_at_end_of_cycle


def get_strengths(list, X_Values):
    final = []
    for i in list:
        final.append(X_Values[i]*i)
    return final


def draw_CRT(X_Values):
    CRT = {}
    center_of_sprite = 0
    y = -1
    for i, value in enumerate(X_Values):
        if i % 40 == 0:
            y+=1
        x = i%40
        v = "#" if abs(x-center_of_sprite) <= 1 else "."
        CRT[(x,y)] = v
        center_of_sprite = X_Values[i]
    return CRT


def print_dict(CRT):
    txt = ""
    for y in range(6):
        for x in range(40):
            txt += CRT[(x,y)]
        txt += "\n"
    return print(txt)




if __name__ == "__main__":
    X_Values = compute_sytem(scans)
    CRT = draw_CRT(X_Values)

    a = print_dict(CRT)
    print(a)
    assert a == """##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."""