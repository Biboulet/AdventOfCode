import os
for i in range(1, 26):
    path = "2022/Exercice/" + str(i) + "/"
    os.mkdir(path)
    open(path + "exerice.py", "a")
    open(path + "input.txt", "a")