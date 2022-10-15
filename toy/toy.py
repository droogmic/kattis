import logging
import os
from dataclasses import dataclass
from math import acos, sin, cos, sqrt
from itertools import permutations

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"))


@dataclass
class Point:
    x: float
    y: float

    def distance(self, other):
        diff_x = self.x - other.x
        diff_y = self.y - other.y
        return sqrt((diff_x * diff_x) + (diff_y * diff_y))

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Point(self.x - other.x, self.y - other.y)

    def __mul__(self, other):
        return Point(self.x * other, self.y * other)

    def __rmul__(self, other):
        return Point(self.x * other, self.y * other)

    def __truediv__(self, other):
        return Point(self.x / other, self.y / other)


def three_sides_given(a, b, c):
    logging.debug(f"{a=} {b=} {c=}")
    num = b * b + c * c - a * a
    den = 2 * b * c
    try:
        return acos(num / den)
    except ValueError:
        logging.error(f"{a=} {b=} {c=}")
        if num - den <= 0.1:
            return 1
        else:
            return 0


class NotRealTriangle(Exception):
    pass


def check_real_triangle(sides):
    assert len(sides) == 3
    if any(p[0] + p[1] < p[2] for p in permutations(sides)):
        raise NotRealTriangle()


def get_new_point(a_point, b_point, top, bottom):

    if a_point.y > b_point.y:
        top_point = a_point
        bottom_point = b_point
    else:
        top_point = b_point
        bottom_point = a_point

    left_len = top_point.distance(bottom_point)
    left_vec = top_point - bottom_point

    check_real_triangle([left_len, top, bottom])

    bottom_angle = three_sides_given(top, bottom, left_len)
    w = bottom * sin(bottom_angle)
    logging.debug(f"{w=}")
    h = bottom * cos(bottom_angle)
    logging.debug(f"{h=}")

    mid = bottom_point + ((h / left_len) * left_vec)
    logging.debug(f"{mid=}")

    try:
        left_gradient = left_vec.y / left_vec.x
    except ZeroDivisionError:
        hor = w * Point(1, 0)
    else:
        try:
            hor_gradient = -1 / left_gradient
        except ZeroDivisionError:
            hor = w * Point(0, 1)
        else:
            logging.debug(f"{hor_gradient=}")
            norm_gradient = sqrt(1 + (hor_gradient * hor_gradient))
            hor = w * Point(1 / norm_gradient, hor_gradient / norm_gradient)
        logging.debug(f"{hor=}")

    new_point = mid + hor
    logging.info(f"{new_point=}")
    return new_point


def get_triangle(a_point, b_point, top, bottom):
    return [a_point, b_point, get_new_point(a_point, b_point, top, bottom)]


def recursive_furthest_right(remaining_segments, next_points):
    if len(remaining_segments) < 2:
        return max(point.x for point in next_points)
    try:
        new_point = get_new_point(
            next_points[0],
            next_points[1],
            remaining_segments.pop(0),
            remaining_segments.pop(0),
        )
    except NotRealTriangle:
        return 0
    furthest_right = (
        recursive_furthest_right(
            remaining_segments=remaining_segments.copy(),
            next_points=(next_points[i], new_point),
        )
        for i in range(2)
    )
    return max(
        *furthest_right,
    )


def furthest_right(segments):
    logging.debug(f"{segments=}")
    next_points = (Point(0, 0), Point(0, segments.pop(0)))
    furthest = recursive_furthest_right(segments, next_points)
    return furthest


def toy(segments):
    logging.info(f"{segments=}")
    best = 0
    for permutation in permutations(segments):
        furthest = furthest_right(list(permutation))
        best = max(best, furthest)
    return best


def main():
    segments, *lengths = (int(v) for v in input().split())
    assert segments == len(lengths)
    print(toy(lengths))


def random_test():
    import random

    random.seed(a=3)

    for n in range(3, 10):
        logging.info(f"{n=}")
        for _ in range(10):
            lengths = [random.randint(1, 99) for _ in range(n)]
            toy(lengths)


def test():
    result = toy([1, 1, 1, 1, 1, 1, 100, 100])
    logging.info(f"{result=}")


if __name__ == "__main__":
    main()
