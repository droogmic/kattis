import logging
import os
from math import sqrt
from dataclasses import dataclass
from enum import Enum

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "WARNING"))


class Outcome(Enum):
    DEAD = "Killed by the impact."
    ALIVE = "James Bond survives."
    STUCK = "Stuck in the air."


@dataclass
class JumpCase:
    k: int
    l: int
    s: int
    w: int

    def is_null(self):
        return self.k == 0 and self.l == 0 and self.s == 0 and self.w == 0

    @property
    def acceleration_freefall(self):
        return 9.81

    @property
    def force_freefall(self):
        return self.acceleration_freefall * self.w

    @property
    def distance_freefall(self):
        return self.l

    @property
    def bridge_height(self):
        return self.s

    @property
    def distance_bungee(self):
        return max(0, self.bridge_height - self.distance_freefall)

    @property
    def mass(self):
        return self.w

    def outcome(self) -> Outcome:
        potential_energy = self.force_freefall * self.bridge_height
        logging.info(f"{potential_energy=}")
        elastic_energy = 0.5 * self.k * self.distance_bungee * self.distance_bungee
        logging.info(f"{elastic_energy=}")
        if elastic_energy > potential_energy:
            return Outcome.STUCK
        fall_velocity = sqrt((potential_energy - elastic_energy) * 2 / self.mass)
        logging.info(f"{fall_velocity=}")
        if fall_velocity > 10:
            return Outcome.DEAD
        return Outcome.ALIVE

    @classmethod
    def from_str(cls, string):
        return cls(*(float(v) for v in string.split()))


def main():
    while True:
        jump = JumpCase.from_str(input())
        if jump.is_null():
            return
        print(jump.outcome().value)


def test():
    import random

    random.seed(1)

    for _ in range(1000):
        jump = JumpCase(
            k=random.uniform(1, 200),
            l=random.uniform(1, 200),
            s=random.uniform(1, 200),
            w=random.uniform(1, 200),
        )
        jump.outcome()


if __name__ == "__main__":
    main()
