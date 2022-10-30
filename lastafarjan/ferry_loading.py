import logging
import os

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"))


class Lane:
    """
    >>> lane = Lane(10)
    >>> assert lane.fits_car(10)
    >>> assert not lane.fits_car(11)
    >>> lane.add_car(5)
    >>> assert lane.fits_car(4)
    >>> assert not lane.fits_car(5)
    >>> new_lane = lane.copy_add_car(1)
    >>> assert lane.fits_car(4)
    >>> assert not new_lane.fits_car(4)
    """

    def __init__(self, length: int):
        self.length = length
        self.remaining = length
        self.cars = []

    def __repr__(self):
        return repr(self.cars)

    def fits_car(self, car) -> bool:
        return car <= self.remaining

    def add_car(self, car) -> None:
        assert self.fits_car(car)
        self.remaining -= car + 1
        self.cars.append(car)

    def copy_add_car(self, car):
        assert self.fits_car(car)
        lane = Lane(self.length)
        lane.remaining = self.remaining - car - 1
        lane.cars = self.cars + [car]
        return lane


def first_fit(cars, length_of_lanes, number_of_lanes=4):
    logging.debug(f"first_fit:{cars=}")
    lanes = [Lane(length=length_of_lanes) for _ in range(number_of_lanes)]
    while cars:
        for lane in lanes:
            if lane.fits_car(cars[0]):
                lane.add_car(cars.pop(0))
                break
        else:
            logging.debug(f"car {cars[0]} cannot fit into {lanes}")
            return lanes, cars
    logging.debug(f"all cars fit")
    return lanes, cars


def sorted_first_fit(cars, length_of_lanes, number_of_lanes=4):
    return first_fit(sorted(cars, reverse=True), length_of_lanes)


def brute_force(cars, length_of_lanes, number_of_lanes=4):
    logging.debug(f"brute_force:{cars=}")
    previous_lanes_permutations = [
        [Lane(length=length_of_lanes) for _ in range(number_of_lanes)]
    ]
    for car in cars:
        logging.debug(f"{car=}:{previous_lanes_permutations=}")
        lanes_permutations = []
        for lane_idx in range(number_of_lanes):
            new_lane_permutations = [p.copy() for p in previous_lanes_permutations]
            for p in new_lane_permutations:
                if p[lane_idx].fits_car(car):
                    p[lane_idx] = p[lane_idx].copy_add_car(car)
            lanes_permutations.extend(new_lane_permutations)
        previous_lanes_permutations = lanes_permutations
    if lanes_permutations:
        return lanes_permutations.pop()
    return None


def binary_search_reverse_first_fit(cars, length_of_lanes, number_of_lanes=4):
    if max(cars) > length_of_lanes:
        return 0

    min_cars_fail = len(cars)
    max_cars_pass = number_of_lanes
    while True:
        curr_num_cars = (max_cars_pass + min_cars_fail + 1) // 2
        curr_cars = cars[:curr_num_cars]
        logging.info(f"{curr_num_cars=}")
        lanes, remaining_cars = sorted_first_fit(curr_cars, length_of_lanes)
        logging.info(f"{lanes=}")
        logging.info(f"{remaining_cars=}")
        if remaining_cars:
            min_cars_fail = curr_num_cars
        else:
            max_cars_pass = curr_num_cars
        if min_cars_fail - max_cars_pass == 1:
            break

    lanes = brute_force(
        cars=cars[: (max_cars_pass - 1)], length_of_lanes=length_of_lanes
    )
    logging.info(f"{lanes=}")

    return max_cars_pass


def main():
    number_of_cars = int(input())
    length_of_lanes = int(input())
    cars = [int(v) for v in input().split()]
    assert len(cars) == number_of_cars
    print(binary_search_reverse_first_fit(cars, length_of_lanes))


if __name__ == "__main__":
    import doctest

    doctest.testmod()

    main()
