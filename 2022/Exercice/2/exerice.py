import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def outcome(a, b):
    if a == b:
        return 3
    if (a == "A" and b == "B") or (a == "B" and b == "C") or (a == "C" and b == "A"):
        return 6
    return 0


def value(b):
    if b == "A":
        return 1
    if b == "B":
        return 2
    if b == "C":
        return 3


def compute_score(scans):
    score = 0
    for line in scans:
        a,b = line.split()
        b = {"X":"A", "Y":"B", "Z":"C"}[b]
        score += outcome(a,b) + value(b)
    return score

def find_complmentary(a, b):
    if b == "X":
        if a == "A":
            return "C"
        if a == "B":
            return "A"
        if a == "C":
            return "B"
    if b == "Y":
        return a
    if b == "Z":
        if a == "A":
            return "B"
        if a == "B":
            return "C"
        if a == "C":
            return "A"


def compute_score2(scans):
    score = 0
    for line in scans:
        a, b = line.split()
        b = find_complmentary(a,b)
        score += outcome(a, b) + value(b)
    return score


if __name__ == "__main__":
    print(compute_score2(scans))

