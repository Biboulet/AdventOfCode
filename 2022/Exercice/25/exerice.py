import os
import utils
import re

scans = utils.read_file(os.getcwd() + "\\input.txt")
Snafu_To_Decimal_Char = {'=':-2 , '-':-1, '0':0, '1':1, '2':2}
Decimal_To_Snafu_Char = {-2:"=" , -1:'-', 0:'0', 1:'1', 2:'2'}


def Snafu_To_Decimal(snafu):
    num = 0
    for i, char in enumerate(snafu[::-1]):
        num += Snafu_To_Decimal_Char[char] * (5**i)
    return num

def Decimal_To_Snafu(decimal):
    snafu = ""
    for i in range(50,-1,-1):
        normalized_between_minus2_and2_num = decimal/5**i
        rounded_num = round(normalized_between_minus2_and2_num)

        if len(snafu) > 0 or rounded_num != 0:
            snafu += Decimal_To_Snafu_Char[rounded_num]
        decimal -= rounded_num*5**i

    return snafu

def solve(scans):
    sum = 0
    for line in scans:
        sum += Snafu_To_Decimal(line)

    print(sum)
    return Decimal_To_Snafu(sum)


if __name__ == "__main__":
    print(solve(scans))
