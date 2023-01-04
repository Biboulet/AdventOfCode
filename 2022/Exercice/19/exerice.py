import os
import utils
import re

scans = utils.read_file(os.getcwd() + "\\input.txt")


class BluePrints:
    def __init__(self, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost):
        self.ore_robot_cost = ore_robot_cost
        self.clay_robot_cost = clay_robot_cost
        self.obsidian_robot_cost = obsidian_robot_cost
        self.geode_robot_cost = geode_robot_cost

# inventory : [r ore, r clay, r obsidian, r geode, ore, clay, obsidian, geode] len = 8
def get_maximum_geode(blueprint, inventory, minutes=0):
    #update les ressources
    inventory[4] += inventory[0]
    inventory[5] += inventory[1]
    inventory[6] += inventory[2]
    inventory[7] += inventory[3]

    if minutes == 15:
        return inventory[7]

    sub_results = []

    #buger pour les
    #on fait rien
    sub_results.append(get_maximum_geode(blueprint, inventory.copy(), minutes+1))
    # on construit un r ore
    if inventory[4] >= blueprint.ore_robot_cost:
        new_inventory = inventory.copy()
        new_inventory[4] -= blueprint.ore_robot_cost
        new_inventory[0] += 1
        sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes+1))

    # on construit un r clay
    if inventory[4] >= blueprint.clay_robot_cost:
        new_inventory = inventory.copy()
        new_inventory[4] -= blueprint.clay_robot_cost
        new_inventory[1] += 1
        sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes+1))

    # on construit un r obsi
    if inventory[4] >= blueprint.obsidian_robot_cost[0] and inventory[5] >= blueprint.obsidian_robot_cost[1]:
        new_inventory = inventory.copy()
        new_inventory[4] -= blueprint.obsidian_robot_cost[0]
        new_inventory[5] -= blueprint.obsidian_robot_cost[1]
        new_inventory[2] += 1
        sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes+1))

    # on construit un r geode
    if inventory[4] >= blueprint.geode_robot_cost[0] and inventory[6] >= blueprint.geode_robot_cost[1]:
        new_inventory = inventory.copy()
        new_inventory[4] -= blueprint.geode_robot_cost[0]
        new_inventory[6] += blueprint.geode_robot_cost[1]
        new_inventory[3] += 1
        sub_results.append(get_maximum_geode(blueprint, new_inventory, minutes+1))

    return max(sub_results)

def get_all_quality_level(blueprints):
    quality_levels = []
    for i, blueprint in enumerate(blueprints):
        print(i)
        quality_levels.append((i+1)*get_maximum_geode(blueprint, [1,0,0,0,0,0,0,0]))
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
