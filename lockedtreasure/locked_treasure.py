import logging
import doctest
from itertools import combinations
from math import factorial

logging.basicConfig(level="INFO")


def choose(a, b) -> int:
    """
    >>> choose(3,1)
    3
    >>> choose(3,2)
    3
    >>> choose(4,1)
    4
    >>> choose(4,2)
    6
    >>> choose(4,3)
    4
    >>> choose(5,1)
    5
    >>> choose(5,2)
    10
    >>> choose(5,3)
    10
    >>> choose(5,4)
    5
    >>> choose(10,3)
    120
    >>> choose(10,6)
    210
    >>> choose(10,7)
    120
    """
    return factorial(a) // (factorial(b) * factorial(a - b))


def solve(n, m) -> int:
    logging.info(f"{n=} {m=}")
    return choose(n, m - 1)


def main() -> None:
    cases = int(input())
    for _ in range(cases):
        n, m = (int(v) for v in input().split())
        assert m <= n
        min_locks = solve(n, m)
        print(str(min_locks))


if __name__ == "__main__":
    doctest.testmod()
    main()
