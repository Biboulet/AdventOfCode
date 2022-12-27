import os
import math

class Vector2:
    def __init__(self, _x, _y):
        self.x = _x
        self.y = _y

    def distance(self, other):
        return math.sqrt((self.x - other.x)**2 + (self.y - other.y)**2)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __add__(self, other):
        return Vector2(self.x+other.x, self.y+ other.y)

    def __str__(self):
        return str(self.x) + " x " + str(self.y)

    def __neg__(self):
        return Vector2(-self.x, -self.y)

    def __hash__(self):
        return (self.x*73856093)^(self.y*19349663)
class Vector3:
    def __init__(self, _x, _y, _z):
        self.x = _x
        self.y = _y
        self.z = _z

    def distance(self, other):
        return math.sqrt((self.x - other.x) ** 2 + (self.y - other.y) ** 2 + (self.z - other.z) ** 2)

    def magnitude(self):
        return self.distance(Vector3(0, 0, 0))

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __add__(self, other):
        return Vector3(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other):
        return Vector3(self.x - other.x, self.y - other.y, self.z - other.z)

    def __str__(self):
        return str(self.x) + " x " + str(self.y) + " x " + str(self.z)

    def __hash__(self):
        return self.x ^ self.y ^ self.z

    def __neg__(self):
        return Vector3(-self.x, -self.y, -self.z)

class Logic:
    @staticmethod
    def Get_Closest_In_List(target, list):
        _list = sorted(list, key=lambda current: math.fabs(target - current))
        return _list[0]

    @staticmethod
    def Closest_in_sorted_list(num, list):
        if num < list[0]: return list[0]

        for i in range(len(list)):
            val = list[i]
            if val > num: return list[i - 1]
        return list[-1]

    @staticmethod
    def BinToDec(octet):
        return sum([2 ** index for index in range(len(octet)) if octet[-(index + 1)] == 1])
    @staticmethod
    def HexToBin(string):
        HexBinDict = {"0": [0, 0, 0, 0], "1": [0, 0, 0, 1],
                      "2": [0, 0, 1, 0], "3": [0, 0, 1, 1],
                      "4": [0, 1, 0, 0], "5": [0, 1, 0, 1],
                      "6": [0, 1, 1, 0], "7": [0, 1, 1, 1],
                      "8": [1, 0, 0, 0], "9": [1, 0, 0, 1],
                      "A": [1, 0, 1, 0], "B": [1, 0, 1, 1],
                      "C": [1, 1, 0, 0], "D": [1, 1, 0, 1],
                      "E": [1, 1, 1, 0], "F": [1, 1, 1, 1]}
        octet = []
        for char in string:
            octet += HexBinDict[char]
        return octet

    @staticmethod
    def get_manathan_distance(vectorList):
        distances = []
        for vec1 in vectorList.values():
            for vec2 in vectorList.values():
                distances.append(abs(vec1.x - vec2.x) + abs(vec1.y - vec2.y) + abs(vec1.z - vec2.z))
        return distances

    @staticmethod
    def get_adjacents_points(index, list, length, height):
        # up
        if not (0 <= index <= length - 1):
            yield list[index - length]
        # up-left
        if not (0 <= index <= length - 1) and not (index % length == 0):
            yield list[index - length - 1]
        # up right
        if not (0 <= index <= length - 1) and not (index % length == length - 1):
            yield list[index - length + 1]
        # down
        if not (length * (height - 1) <= index <= length * height):
            yield list[index + length]
        # down left
        if not (length * (height - 1) <= index <= length * height) and not (index % length == 0):
            yield list[index + length - 1]
        # down right
        if not (length * (height - 1) <= index <= length * height) and not (index % length == length - 1):
            yield list[index + length + 1]
        # right
        if not (index % length == length - 1):
            yield list[index + 1]
        # left
        if not (index % length == 0):
            yield list[index - 1]

    @staticmethod
    def dijkstra(graph, target,):
        # Initialisation
        local_graph = []
        for node in graph.nodes:
            node.cost_to_go = 9999999
            local_graph.append(node)
        local_graph[0].cost_to_go = 0

        while local_graph:
            #get_closest_node
            closest_node = local_graph[0]
            for node in local_graph:
                if node.cost_to_go < closest_node.cost_to_go:
                    closest_node = node

            #pour finir
            if closest_node is target:
                return closest_node.cost_to_go

            local_graph.remove(closest_node)

            #on update les linked
            for neighbour in closest_node.linked:
                neighbour_cost = closest_node.cost_to_go + neighbour.value
                if neighbour_cost < neighbour.cost_to_go:
                    neighbour.cost_to_go = neighbour_cost

class Math:
    @staticmethod
    def Average(list):
        return sum(list)/len(list)

    @staticmethod
    def QuadraticAverage(list):
        return math.sqrt(sum([v**2 for v in list]) / len(list))

    @staticmethod
    def Mediane(list):
        list.sort()
        return list[len(list)//2]

class Node:
    def __init__(self, value):
        self.linked = []
        self.value_to_go = 0
        self.value = value

    def __str__(self):
        return str(self.value)

class Tree:
    def __init__(self):
        self.nodes = []

def read_file(path):
    if os.path.isfile(path):
        return open(path, "r").read().splitlines()
    print("File does not exist")
