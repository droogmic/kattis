import logging
import os

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"))


class Lane:
    def __init__(self, length: int):
        self.length = length
        self.remaining = length
        self.cars = set()

    def __repr__(self):
        return repr(self.cars)

    def fits_car(self, car):
        return car <= self.remaining

    def add_car(self, car):
        assert self.fits_car(car)
        self.remaining -= car + 1
        self.cars.append(car)


def first_fit(cars, length_of_lanes, number_of_lanes=4):
    lanes = [Lane(length=length_of_lanes) for _ in range(number_of_lanes)]
    while cars:
        car = cars.pop(0)
        for lane in lanes:
            if lane.fits_car(car):
                lane.add_car(car)
                break
        else:
            return lanes, cars
    return lanes, None


def sorted_first_fit(cars, length_of_lanes, number_of_lanes=4):
    return first_fit(sorted(cars, reverse=True), length_of_lanes)


def binary_search_reverse_first_fit(cars, length_of_lanes, number_of_lanes=4):
    if max(cars) > length_of_lanes:
        return 0

    best_max_num_cars = max_num_cars
    best_min_num_cars = number_of_lanes
    while True:
        curr_num_cars = (min_num_cars + max_num_cars) / 2
        curr_cars = cars[:curr_num_cars]
        logging.info(f"{curr_num_cars=}")
        lanes, remaining_cars = sorted_first_fit(curr_cars, length_of_lanes)
        logging.info(f"{lanes=}")
        if remaining_cars is None:
            best_max_num_cars = curr_num_cars
        else:
            best_min_num_cars = curr_num_cars
        if best_max_num_cars - best_min_num_cars == 1:
            break

    return best_max_num_cars


def main():
    number_of_cars = int(input())
    length_of_lanes = int(input())
    cars = [int(v) for v in input().split()]
    assert len(cars) == number_of_cars
    print(binary_search_reverse_first_fit(cars, length_of_lanes))


if __name__ == "__main__":
    main()
