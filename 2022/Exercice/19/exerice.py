import os
import utils
import re
import time

start_time = time.time()
scans = utils.read_file(os.getcwd() + "\\input.txt")


# LISTE D'OPTI:
#   - pas plus d'expemplaire d'un robot de type x que le montant maximal dépensable de la ressource x par tour
#   - crée une mémoire
#   - quand ne ressource de trop grande taille (+63) break

class BluePrints:
    def __init__(self, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost):
        self.ore_robot_cost = ore_robot_cost
        self.clay_robot_cost = clay_robot_cost
        self.obsidian_robot_cost_ore = obsidian_robot_cost[0]
        self.obsidian_robot_cost_clay = obsidian_robot_cost[1]
        self.geode_robot_cost_ore = geode_robot_cost[0]
        self.geode_robot_cost_obsidian = geode_robot_cost[1]

        self.max_robot_ore = max(self.ore_robot_cost, self.clay_robot_cost, self.obsidian_robot_cost_ore, self.geode_robot_cost_ore)
        self.max_robot_clay = self.obsidian_robot_cost_clay
        self.max_robot_obsidian = self.geode_robot_cost_obsidian


class Inventory:
    def __init__(self, ore=0, clay=0, obsidian=0, geode=0, robot_ore=0, robot_clay=0, robot_obsidian=0, robot_geode=0):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian
        self.geode = geode

        self.ore_robot = robot_ore
        self.clay_robot = robot_clay
        self.obsidian_robot = robot_obsidian
        self.geode_robot = robot_geode

    def copy(self):
        return Inventory(self.ore, self.clay, self.obsidian, self.geode, self.ore_robot, self.clay_robot,
                         self.obsidian_robot, self.geode_robot)

    def is_valid(self):
        return self.ore < 64 and self.clay < 64 and self.obsidian < 64

    def buy_robot_ore(self, blueprint: BluePrints):
        self.ore -= blueprint.ore_robot_cost
        self.ore_robot += 1
        return self

    def buy_robot_clay(self, blueprint: BluePrints):
        self.ore -= blueprint.clay_robot_cost
        self.clay_robot += 1
        return self

    def buy_robot_obsidian(self, blueprint: BluePrints):
        self.ore -= blueprint.obsidian_robot_cost_ore
        self.clay -= blueprint.obsidian_robot_cost_clay
        self.obsidian_robot += 1
        return self

    def buy_robot_geode(self, blueprint: BluePrints):
        self.ore -= blueprint.geode_robot_cost_ore
        self.obsidian -= blueprint.geode_robot_cost_obsidian
        self.geode_robot += 1
        return self

    def can_buy_ore_robot(self, blueprint: BluePrints):
        return self.ore >= blueprint.ore_robot_cost and self.ore_robot < blueprint.max_robot_ore

    def can_buy_clay_robot(self, blueprint: BluePrints):
        return self.ore >= blueprint.clay_robot_cost and self.clay_robot < blueprint.max_robot_clay

    def can_buy_obsidian_robot(self, blueprint: BluePrints):
        return self.ore >= blueprint.obsidian_robot_cost_ore and self.clay >= blueprint.obsidian_robot_cost_clay and self.obsidian_robot < blueprint.max_robot_obsidian

    def can_buy_geode_robot(self, blueprint: BluePrints):
        return self.ore >= blueprint.geode_robot_cost_ore and self.obsidian >= blueprint.geode_robot_cost_obsidian

    def update(self):
        self.ore += self.ore_robot
        self.clay += self.clay_robot
        self.obsidian += self.obsidian_robot
        self.geode += self.geode_robot
        return self

    def get_key(self, minutes):
        # 25 possibilité pr les minutes (5 bits)
        # 8^3 possibilité de robot (9 bits)
        # 64^3 possibilité de ressources, a baisser peut etre pr l'obsi et l'ore (18 bits)
        # ou 32ore * 64clay * 32 obsi (16bits)

        robot_possibility = self.ore_robot + self.clay_robot*2**3 + self.obsidian_robot*2**7
        ressource_possibility = self.ore + self.clay *2**6 + self.obsidian * 2**12
        return minutes + robot_possibility*2**5 + ressource_possibility*2**15


def get_maximum_geode(blueprint: BluePrints, inventory: Inventory, minutes, DP):
    if minutes == 0:
        return inventory.geode

    if not inventory.is_valid():
        return 0

    key = inventory.get_key(minutes)
    if DP[key] is not None:
        return DP[key]


    sub_results = []

    if inventory.can_buy_geode_robot(blueprint):
        new_inventory = inventory.copy()
        new_inventory.update()
        new_inventory.buy_robot_geode(blueprint)

        sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes - 1, DP))

    else:
        sub_results.append(get_maximum_geode(blueprint, inventory.copy().update(), minutes - 1, DP))

        if inventory.can_buy_ore_robot(blueprint):
            new_inventory = inventory.copy()
            new_inventory.update()
            new_inventory.buy_robot_ore(blueprint)
            sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes - 1, DP))

        if inventory.can_buy_clay_robot(blueprint):
            new_inventory = inventory.copy()
            new_inventory.update()
            new_inventory.buy_robot_clay(blueprint)
            sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes - 1, DP))

        if inventory.can_buy_obsidian_robot(blueprint):
            new_inventory = inventory.copy()
            new_inventory.update()
            new_inventory.buy_robot_obsidian(blueprint)
            sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes - 1, DP))

    DP[key] = max(sub_results)
    return DP[key]


def get_all_quality_level(blueprints):
    quality_levels = []
    for i, blueprint in enumerate(blueprints):
        print(i)
        DP = [None] * 2**32
        inventory = Inventory(robot_ore=1)
        quality_levels.append((i + 1) * get_maximum_geode(blueprint, inventory, 24, DP))
        print(quality_levels)
        del DP

    return quality_levels


def parse_input(scans):
    blueprints = []
    for line in scans:
        args = line.split(".")
        ore_robot_cost = int(re.search('robot costs (.*) ore', args[0]).group(1))

        clay_robot_cost = int(re.search('robot costs (.*) ore', args[1]).group(1))

        obsidian_robot_cost_ore = int(re.search('robot costs (.*) ore', args[2]).group(1))
        obsidian_robot_cost_clay = int(re.search('and (.*) clay', args[2]).group(1))

        geode_robot_cost_ore = int(re.search('robot costs (.*) ore', args[3]).group(1))
        geode_robot_cost_obsedian = int(re.search('and (.*) obsidian', args[3]).group(1))

        blueprints.append(
            BluePrints(ore_robot_cost, clay_robot_cost, (obsidian_robot_cost_ore, obsidian_robot_cost_clay),
                       (geode_robot_cost_ore, geode_robot_cost_obsedian)))

    return blueprints


if __name__ == "__main__":
    blueprints = parse_input(scans)
    quality_levels = get_all_quality_level(blueprints)
    print(sum(quality_levels))
    print("--- %s seconds ---" % (time.time() - start_time))
