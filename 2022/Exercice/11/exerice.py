import math
import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")
product_of_modulos_value = 0


class Monkey:
    def __init__(self, items, is_add, value_operation, value_modulo, monkey_outcome_true, monkeyOutcome_false):
        self.activity_score = 0
        self.items = items
        self.is_add = is_add
        self.value_operation = value_operation
        self.value_modulo = value_modulo
        self.monkey_outcomeTrue = monkey_outcome_true
        self.monkey_outcomeFalse = monkeyOutcome_false

    # returns the new value of the item + the monkey whom to give
    def inspect_item(self, item):
        self.activity_score += 1
        self.items.remove(item)

        val = int(self.value_operation) if self.value_operation.isdigit() else item

        item = item + val if self.is_add else item * val
        item = reduce_item(item)

        return item, self.monkey_outcomeTrue if item % self.value_modulo == 0 else self.monkey_outcomeFalse


def parse_input(scans):
    monkeys = []
    for i in range(0, len(scans) + 1, 7):
        items = [int(num) for num in scans[i + 1].split(": ")[1].split(", ")]
        is_add = scans[i + 2].count("+") == 1
        value_operation = scans[i + 2].split("old")[1][3:]
        value_modulo = int(scans[i + 3].split("by ")[1])
        outcome_true = int(scans[i + 4].split("monkey ")[1])
        outcome_false = int(scans[i + 5].split("monkey ")[1])
        monkey = Monkey(items, is_add, value_operation, value_modulo, outcome_true, outcome_false)
        monkeys.append(monkey)
    return monkeys

def reduce_item(item):
    return item % product_of_modulos_value

def monkey_buisness(monkeys):
    monkeys.sort(key=lambda monkey: monkey.activity_score, reverse=True)
    return monkeys[0].activity_score * monkeys[1].activity_score


def simulate_rounds(round_count, monkeys):
    for i in range(round_count):
        for monkey in monkeys:
            for item in monkey.items.copy():
                new_item, monkey_to_give = monkey.inspect_item(item)
                monkeys[monkey_to_give].items.append(new_item)


if __name__ == "__main__":
    monkeys = parse_input(scans)
    product_of_modulos_value = math.prod([monkey.value_modulo for monkey in monkeys])
    simulate_rounds(10000, monkeys)
    print(monkey_buisness(monkeys))
