import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def parse_packet(line):
    num = []

    if len(line) == 2:
        return num

    current_item = ""
    depth = 0
    for char in line[1:-1]:
        if depth == 0 and char == ",":
            num.append(current_item)
            current_item = ""
            continue

        current_item += char

        if char == "[":
            depth += 1
        elif char == "]":
            depth -= 1
    num.append(current_item)

    for i, item in enumerate(num):
        if item.isdigit():
            num[i] = int(item)
        else:
            num[i] = parse_packet(item)
    return num


def parse_input(scans):
    pairs = []

    for i in range(0, len(scans), 3):
        packet_1 = parse_packet(scans[i])
        packet_2 = parse_packet(scans[i + 1])
        pairs.append((packet_1, packet_2))

    return pairs


def is_valid(first_packet, second_packet):
    for i in range(min([len(first_packet), len(second_packet)])):
        a = first_packet[i]
        b = second_packet[i]

        if type(a) == int and type(b) == int:
            if a != b:
                return a < b

        elif type(a) == list and type(b) == list:
            result = is_valid(a,b)
            if result is not None:
                return result
        else:
            if type(a) == list:
                b = [b]
            elif type(b) == list:
                a = [a]
            result = is_valid(a, b)
            if result is not None:
                return result

    if len(first_packet) == len(second_packet):
        return None
    return len(first_packet) < len(second_packet)


def compare_all_pairs(pairs):
    count = 0

    for i, packets in enumerate(pairs):
        first_packet = packets[0]
        second_packet = packets[1]
        if is_valid(first_packet, second_packet):
            count += i + 1
    return count


if __name__ == "__main__":
    pairs = parse_input(scans)
    print(compare_all_pairs(pairs))
