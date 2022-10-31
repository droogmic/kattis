import logging
import os

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"))


def solve(total_distance, total_volume, capacity):
    distance = 0
    volume = total_volume
    while True:
        if volume <= 0:
            return 0.0
        if distance >= total_distance:
            return volume
        if volume <= capacity:
            volume -= total_distance - distance
            return max(volume, 0.0)
        part_volume = volume % capacity
        if part_volume == 0:
            part_volume = capacity
        trips = -(volume // -capacity)
        part_distance = part_volume / (2 * trips - 1)
        logging.info(f"{distance=} {part_distance=} {volume=} {part_volume=}")
        distance += part_distance
        volume -= part_volume


def main():
    distance, total, capacity = (float(v) for v in input().split())
    print(solve(distance, total, capacity))


if __name__ == "__main__":
    main()
