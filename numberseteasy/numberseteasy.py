import logging

logging.basicConfig(level="ERROR")


def prime_factors(v: int) -> set:
    """
    >>> prime_factors(12)
    {2, 3}
    """
    res = []
    for i in range(2, v):
        if v % i != 0:
            continue
        return prime_factors(v // i).union([i])
    return {v}


from dataclasses import dataclass


@dataclass
class Group:
    primes: set
    values: list


def calculate(range_vals, min_prime) -> int:
    factors = {
        v: set(p for p in prime_factors(v) if p >= min_prime)
        for v in range(range_vals[0], range_vals[1])
    }
    logging.debug("%s", factors)

    groups = []
    flag = False
    while any(
        val not in [v for group in groups for v in group.values] for val in factors
    ):
        for val, primes in factors.items():
            if val in [v for group in groups for v in group.values]:
                continue
            for p in primes:
                for group in groups:
                    if p in group.primes:
                        group.primes.update(primes)
                        group.values.append(val)
                        break
                else:
                    continue
                break
            else:
                if len(primes) == 0 or val in primes or flag:
                    groups.append(Group(primes, [val]))
                    break
        else:
            flag = True
            continue
        flag = False
    logging.debug("%s", groups)
    return len(groups)


def main():
    cases = int(input())
    for idx in range(cases):
        start, end, prime = (int(v) for v in input().split())
        print(f"Case #{idx + 1}: {calculate((start, end), prime)}")


if __name__ == "__main__":
    import doctest

    doctest.testmod()
    main()
