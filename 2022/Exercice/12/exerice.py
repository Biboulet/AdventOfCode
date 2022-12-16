import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


class Node:
    def __init__(self, v):
        self.cost_to_go = 0
        self.value = v
        self.is_start = False
        self.is_end = False
        self.linked = []


def get_adjacent_2D(scans, key):
    x, y = key
    coords = []
    # up
    if not y == 0:
        coords.append((x, y - 1))
    # down
    if not y == len(scans) - 1:
        coords.append((x, y + 1))
    # right
    if not x == len(scans[0]) - 1:
        coords.append((x + 1, y))
    # left
    if not x == 0:
        coords.append((x - 1, y))
    return coords


def parse_input(scans):
    map = {}
    for y, line in enumerate(scans):
        for x, char in enumerate(line):
            node = Node(ord(char))

            if ord(char) == ord("S"):
                node.is_start = True
                node.value = ord("a")

            elif ord(char) == ord("E"):
                node.is_end = True
                node.value = ord("z")

            if node.value == ord("a"):
                if 0 <= x <= 3: node.is_start = True

            map[(x, y)] = node

    nodes = []
    for key, node in map.items():
        coords_adj = get_adjacent_2D(scans, key)

        node.linked = [map[coord] for coord in coords_adj if map[key].value + 1 >= map[coord].value]
        nodes.append(node)

    return nodes


def dijkstra(graph, start_index):
    # Initialisation
    local_graph = []
    for node in graph:
        node.cost_to_go = 9999999
        local_graph.append(node)

    local_graph[start_index].cost_to_go = 0

    target_index = [graph.index(node) for node in local_graph if node.is_end][0]
    target = local_graph[target_index]

    while local_graph:
        # get_closest_node
        closest_node = local_graph[0]
        for node in local_graph:
            if node.cost_to_go < closest_node.cost_to_go:
                closest_node = node

        # pour finir
        if closest_node is target:
            return closest_node.cost_to_go

        local_graph.remove(closest_node)

        # on update les linked
        for neighbour in closest_node.linked:
            neighbour_cost = closest_node.cost_to_go + 1
            if neighbour_cost < neighbour.cost_to_go:
                neighbour.cost_to_go = neighbour_cost


def solve(nodes):
    different_start_steps = []
    all_a = [nodes.index(node) for node in nodes if node.is_start]
    i = 0
    print(len(all_a))
    for a_index in all_a:
        i += 1
        print(i)
        different_start_steps.append(dijkstra(nodes, a_index))
    return min(different_start_steps)


if __name__ == "__main__":
    nodes = parse_input(scans)
    print(solve(nodes))
