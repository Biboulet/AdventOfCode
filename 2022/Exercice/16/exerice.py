import os
import utils
from itertools import combinations

scans = utils.read_file(os.getcwd() + "\\input.txt")
valves_pressure = []
valves_linked = []
distance_between_valves = {}
DP = []


# rajouter des s
def parse_input(scans):
    value_to_index = [line.split(" has ")[0].split()[1] for line in scans]

    pressure = []
    for line in scans:
        linked_valves = line.split("valves ")[1]
        linked_valves = [linked_valves] if len(linked_valves) == 2 else linked_valves.split(", ")

        valves_linked.append([value_to_index.index(valve) for valve in linked_valves])

        flowRate = int(line.split("flow rate=")[1].split(";")[0])
        pressure.append(flowRate)

    important_valves = [i for i, pressure in enumerate(pressure) if pressure != 0]
    important_valves.insert(0, value_to_index.index("AA"))
    get_all_distance(important_valves)
    valves_pressure.extend([pressure[i] for i in important_valves])


def dijkstrat_distance(source, dest):
    dist = {i: 999 for i in range(len(scans))}
    dist[source] = 0
    graph = list(range(len(scans)))

    while graph:
        closest_valve = min(graph, key=lambda vertex: dist[vertex])
        graph.remove(closest_valve)

        if closest_valve == dest:
            return dist[closest_valve]

        for neighbour in valves_linked[closest_valve]:
            if dist[closest_valve] + 1 < dist[neighbour]:
                dist[neighbour] = dist[closest_valve] + 1


def get_all_distance(important_valves):
    for new_i, old_i in enumerate(important_valves):
        for new_i2, old_i2 in enumerate(important_valves):
            distance_between_valves[(new_i, new_i2)] = dijkstrat_distance(old_i, old_i2)

    return distance_between_valves


def get_max_pressure(curr_valve, openned_valves, minutes, player_after):
    if minutes <= 0:
        return 0 if player_after <= 0 else get_max_pressure(0, openned_valves, 26, player_after - 1)

    # 15**2 combinaisons de pressure ouverte (15bit)
    # 16 pos possible (4 bit)
    # 27 min possible (5 bit)
    # 2 player possible (1 bit)
    key = player_after + minutes*2 + curr_valve*2*32 + openned_valves*2**10
    if DP[key] is not None:
        return DP[key]

    # open_current_valve
    best_sub_pressure = 0
    bit_mask_val = 2**(curr_valve-1)
    if curr_valve != 0 and bit_mask_val & openned_valves == 0:
        best_sub_pressure = max(best_sub_pressure, valves_pressure[curr_valve] * (minutes - 1) +
                                get_max_pressure(curr_valve, openned_valves | bit_mask_val, minutes - 1, player_after))

    # move to another pressure
    for valve in range(1,len(valves_pressure)):
        if valve != curr_valve:
            best_sub_pressure = max(best_sub_pressure, get_max_pressure(valve, openned_valves, minutes - distance_between_valves[(curr_valve, valve)], player_after))

    DP[key] = best_sub_pressure
    return best_sub_pressure


if __name__ == "__main__":
    parse_input(scans)

    DP = [None] * 2 ** 25

    print(get_max_pressure(0, 0, 26, 1))
