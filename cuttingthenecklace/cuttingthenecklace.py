import logging
import os

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "WARNING"))


def split_start(weights, target_weight) -> bool:
    rolling_sum = 0
    for w in weights:
        rolling_sum += w
        if rolling_sum < target_weight:
            continue
        if rolling_sum == target_weight:
            rolling_sum = 0
            continue
        if rolling_sum > target_weight:
            return False
    return True


def split(friends, weights) -> bool:
    sum_weights = sum(weights)
    if sum_weights % friends != 0:
        return False

    target_weight = sum_weights // friends

    largest_weight = max(weights)
    if largest_weight > target_weight:
        return False

    starting_weights = []
    while sum(starting_weights) < target_weight:
        if split_start(weights=weights, target_weight=target_weight):
            return True
        starting_weights.append(weights.pop(0))

    return False


def main():
    k, n = (int(v) for v in input().split())
    weights = [int(v) for v in input().split()]
    assert len(weights) == n
    if split(k, weights):
        print("YES")
    else:
        print("NO")


if __name__ == "__main__":
    main()
