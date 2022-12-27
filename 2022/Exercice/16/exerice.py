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

    if minutes == 30:
        return pressure

    sub_pressures = []
    if current_valve not in openned_valves and valves_pressure[current_valve] != 0:
        sub_pressures.append(get_pressure_opening(current_valve, openned_valves + [current_valve], minutes + 1))

    for neighbour in valves_linked[current_valve]:
        sub_pressures.append(get_pressure_opening(neighbour, openned_valves, minutes+1))

    all_best_pressures[(current_valve, tuple(openned_valves), minutes)] = pressure + max(sub_pressures)
    return pressure + max(sub_pressures)


if __name__ == "__main__":
    valves_pressure, valves_linked = parse_input(scans)
    print(get_pressure_opening("AA", []))


