import logging
import os
from itertools import combinations, permutations

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"))


def solve():
    raise NotImplementedError()


def dijkstra(rails):
    neighbourhoods = set(r for pair in rails for r in pair)
    edges = {}
    for pair in rails:
        edges.setdefault(pair[0], set()).add(pair[1])
        edges.setdefault(pair[1], set()).add(pair[0])
    logging.info(f"{edges=}")
    shortest = {v: None for v in neighbourhoods}
    for neighbourhood in neighbourhoods:
        unvisited = neighbourhoods.copy()

    logging.info(f"{shortest=}")


def main():
    n, m, k = (int(v) for v in input().split())
    rails = [input().split() for _ in range(n - 1)]
    logging.info(f"{rails=}")
    ministers = [[int(v) for v in input().split()][1:] for _ in range(m)]
    dijkstra(rails)


if __name__ == "__main__":
    main()
