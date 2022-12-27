import os
import utils

scans = utils.read_file(os.getcwd() + "\\input.txt")


def get_non_beacons_position_count(sensors, y):
    y_row = {}
    for curr_sensor in sensors:
        manath = curr_sensor.manathan_dist_with_beacon()
        dist_sensors_y_row = abs(curr_sensor.pos[1] - y)
        difference = manath - dist_sensors_y_row
        if difference < 0:
            continue
        elif difference == 1:
            if curr_sensor.pos[0] != curr_sensor.beacon_pos[0]:
                y_row[curr_sensor.pos[0]] = 1
        else:
            for x in range(curr_sensor.pos[0] - difference, curr_sensor.pos[0] + difference + 1):
                if (x, y) != curr_sensor.beacon_pos:
                    y_row[x] = 1

    return len(y_row)

class Sensors:
    def __init__(self, pos, beacon_pos):
        self.pos = pos
        self.beacon_pos = beacon_pos

    def get_points_around(self):
        around_points = set()
        dist_pos_beacon_pos = manath(self.pos, self.beacon_pos)
        Xmin = self.pos[0] - dist_pos_beacon_pos - 1
        Xmax = self.pos[0] + dist_pos_beacon_pos + 1

        i = 0
        for x in range(Xmin,Xmax+1):
            around_points.add((x,self.pos[1]+i))
            around_points.add((x,self.pos[1]-i))
            i+=1 if x < self.pos[0] else -1
        return list(around_points)

def parse_input(scans):
    sensors = []
    for line in scans:
        halves = line.split(":")

        args1 = halves[0].split(", y=")
        _pos = (int(args1[0].split("x=")[1]), int(args1[1]))

        args2 = halves[1].split(", y=")
        _beacon_pos = (int(args2[0].split("x=")[1]), int(args2[1]))

        sensors.append(Sensors(_pos, _beacon_pos))
    return sensors


def get_distress_beacon(sensors, max):
    available_pos = []
    print(len(sensors))
    for i, curr_sensor in enumerate(sensors):
        print(i)
        available_pos.extend(curr_sensor.get_points_around())

    valid_points = [pos for pos in available_pos if 0 < pos[0] < max and 0 < pos[1] < max]

    print(len(valid_points))
    for i,point in enumerate(valid_points):
        print(i)
        if all([manath(point, curr_sensor.pos) > manath(curr_sensor.beacon_pos, curr_sensor.pos) for curr_sensor in sensors]):
            return point



def manath(tuple1, tuple2):
    return abs(tuple1[0] - tuple2[0]) + abs(tuple1[1] - tuple2[1])


if __name__ == "__main__":
    sensors = parse_input(scans)
    distress_beacon = get_distress_beacon(sensors, 4000000)
    print(distress_beacon[0] * 4000000 + distress_beacon[1])
