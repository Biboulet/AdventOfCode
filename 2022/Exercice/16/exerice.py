import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")
valves_pressure = {}
valves_linked = {}
all_best_pressures = {}


# rajouter des s
def parse_input(scans):
    pressure = {}
    linked = {}
    for line in scans:
        valve_name = line.split(" has ")[0].split()[1]
        flowRate = int(line.split("flow rate=")[1].split(";")[0])
        linked_valves = line.split("valves ")[1]
        linked_valves = [linked_valves] if len(linked_valves) == 2 else linked_valves.split(", ")
        pressure[valve_name] = flowRate
        linked[valve_name] = linked_valves

    return pressure, linked


def get_pressure_opening(current_valve, openned_valves, minutes=1, pressure=0):
    if (current_valve, tuple(openned_valves), minutes) in all_best_pressures.keys():
        return all_best_pressures[(current_valve, tuple(openned_valves), minutes)]

    pressure += sum([valves_pressure[openned] for openned in openned_valves])

    if minutes == 26:
        return pressure

    sub_pressures = []
    if current_valve not in openned_valves and valves_pressure[current_valve] != 0:
        sub_pressures.append(get_pressure_opening(current_valve, openned_valves + [current_valve], minutes + 1))

    for neighbour in valves_linked[current_valve]:
        sub_pressures.append(get_pressure_opening(neighbour, openned_valves, minutes + 1))

    all_best_pressures[(current_valve, tuple(openned_valves), minutes)] = pressure + max(sub_pressures)
    return pressure + max(sub_pressures)


def dijkstrat_distance(source, dest):
    dist = {valve: 999 for valve in valves_linked.keys()}
    dist[source] = 0
    graph = list(valves_linked.keys())

    while graph:
        closest_valve = min(graph, key=lambda vertex: dist[vertex])
        graph.remove(closest_valve)

        if closest_valve == dest:
            return dist[closest_valve]

        for neighbour in valves_linked[closest_valve]:
            if dist[closest_valve] + 1 < dist[neighbour]:
                dist[neighbour] = dist[closest_valve] + 1


def get_all_distance():
    distance_between_valves = {}
    important_valves = ["AA"] + [key for key, val in valves_pressure.items() if val != 0]

    for valve in important_valves:
        for valve2 in important_valves:
            distance_between_valves[(valve, valve2)] = dijkstrat_distance(valve, valve2)

    return distance_between_valves


def get_pressure_openned(first_player, second_player, distance_between_valves):
    total = 0

    for valve_list in [first_player, second_player]:
        remaining_minutes = 26
        last_valve = "AA"
        for valve in valve_list:
            remaining_minutes -= distance_between_valves[(last_valve, valve)] + 1
            if remaining_minutes <= 0:
                break
            total += valves_pressure[valve] * remaining_minutes
            last_valve = valve

    return total


def get_best_next_position(distance_between_valves, p1_pos, important_valves, i):
    max_pressure = 0
    pos = None

    for valve in important_valves:
        pressure_openned = valves_pressure[valve] * (26 - i - distance_between_valves[(p1_pos, valve)] - 1)
        if pressure_openned > max_pressure:
            max_pressure = pressure_openned
            pos = valve

    return pos

def get_max_pressure_released_p2(distance_between_valves):
    important_valves = [key for key, val in valves_pressure.items() if val != 0]

    pressure = 0
    p1_pos = "AA"
    p2_pos = "AA"
    p1_next_turn = 0
    p2_next_turn = 0

    for i in range(26):
        if i == p1_next_turn:

            pressure += valves_pressure[p1_pos] * (26-i)

            new_pos = get_best_next_position(distance_between_valves, p1_pos, important_valves, i)
            if new_pos is not None:
                p1_next_turn += distance_between_valves[(p1_pos, new_pos)] + 1
                p1_pos = new_pos
                important_valves.remove(new_pos)

        if i == p2_next_turn:
            pressure += valves_pressure[p2_pos] * (26 - i)

            new_pos = get_best_next_position(distance_between_valves, p2_pos, important_valves, i)
            if new_pos is not None:
                p2_next_turn += distance_between_valves[(p2_pos, new_pos)] + 1
                p2_pos = new_pos
                important_valves.remove(new_pos)

    return pressure




if __name__ == "__main__":
    valves_pressure, valves_linked = parse_input(scans)
    distance_between_valves = get_all_distance()
    print(get_max_pressure_released_p2(distance_between_valves))
