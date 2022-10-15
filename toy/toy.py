import logging
from dataclasses import dataclass
from math import acos, sin, cos, sqrt
from itertools import permutations

logging.basicConfig(level="INFO")


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
    return acos((b * b + c * c - a * a) / (2 * b * c))


class NotRealTriangle(Exception):
    pass


def get_triangle(a_point, b_point, top, bottom):

    if a_point.y > b_point.y:
        top_point = a_point
        bottom_point = b_point
    else:
        top_point = b_point
        bottom_point = a_point

    left_len = top_point.distance(bottom_point)
    left_vec = top_point - bottom_point

    if sum(
        v for v in (top, bottom, left_len) if v != max(top, bottom, left_len)
    ) <= max(top, bottom, left_len):
        raise NotRealTriangle()

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
        hor_gradient = 0
    else:
        hor_gradient = -1 / left_gradient
    logging.debug(f"{hor_gradient=}")
    norm_gradient = sqrt(1 + (hor_gradient * hor_gradient))
    hor = w * Point(1 / norm_gradient, hor_gradient / norm_gradient)
    logging.debug(f"{hor=}")

    triangle = [bottom_point, top_point, mid + hor]
    logging.debug(f"{triangle=}")

    return triangle


def furthest_right():
    get_triangle(points.pop(), points.pop(), segments.pop(), segments.pop())


def toy(segments):
    best = 0
    for permutation in permutations(segments):
        segments = list(permutation)
        logging.debug(f"{segments=}")
        try:
            structure = [
                get_triangle(
                    Point(0, 0),
                    Point(0, segments.pop()),
                    segments.pop(),
                    segments.pop(),
                )
            ]
        except NotRealTriangle:
            continue
        while len(segments) >= 2:
            last_triangle = structure[-1]
            next_triangles = []
            for points in permutations(last_triangle):
                points = list(points)
                try:
                    next_triangles.append(
                        get_triangle(
                            points.pop(), points.pop(), segments.pop(), segments.pop()
                        )
                    )
                except NotRealTriangle:
                    continue
            next_triangle = max(*next_triangles, key=lambda t: max(p.x for p in t))
            structure.append(triangle)
            logging.debug(f"{structure=}")
        logging.info(f"{structure=}")
        furthest = max(max(p.x for p in triangle) for triangle in structure)
        best = max(best, furthest)
    return best


def main():
    segments, *lengths = (int(v) for v in input().split())
    assert segments == len(lengths)
    print(toy(lengths))


if __name__ == "__main__":
    main()
