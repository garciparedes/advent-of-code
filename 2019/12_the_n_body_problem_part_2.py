from __future__ import annotations

import re
from copy import deepcopy
from functools import reduce
from pathlib import Path
from sys import argv
from typing import Tuple, List, Set
import itertools as it


def gcd(a, b):
    """Compute the greatest common divisor of a and b"""
    while b > 0:
        a, b = b, a % b
    return a


def lcm(a, b):
    """Compute the lowest common multiple of a and b"""
    return a * b / gcd(a, b)


class Moon(object):
    def __init__(self, position: Tuple[int, ...]):
        self.position = list(position)
        self.velocity = list(0 for _ in range(self.dimensions))

    @property
    def dimensions(self) -> int:
        return len(self.position)

    @property
    def energy(self) -> int:
        return sum(abs(v) for v in self.position) * sum(abs(v) for v in self.velocity)

    def update_velocity(self, other: Moon) -> None:
        for idx in range(self.dimensions):
            if self.position[idx] < other.position[idx]:
                self.velocity[idx] += 1
            elif self.position[idx] > other.position[idx]:
                self.velocity[idx] -= 1

    def update_position(self) -> None:
        for idx in range(self.dimensions):
            self.position[idx] += self.velocity[idx]

    def __str__(self):
        return f'Moon({self.position},{self.velocity})'

    def __repr__(self):
        return str(self)

    def __eq__(self, other: Moon) -> bool:
        return self.position == other.position and self.velocity == other.velocity

    def __hash__(self):
        return hash(tuple(self.position + self.velocity))


def simulate_step(moons: Tuple[Moon], steps: int = 1) -> Tuple[Moon]:
    moons = deepcopy(moons)
    for _ in range(steps):
        for a, b in it.permutations(moons, 2):
            a.update_velocity(b)
        for moon in moons:
            moon.update_position()
    return tuple(moons)


def has_cycle_at(one: Tuple[Moon], two: Tuple[Moon], i: int) -> bool:
    return all(
        a.position[i] == b.position[i] and a.velocity[i] == b.velocity[i]
        for a, b in zip(one, two)
    )


def find_cycle_steps(moons: Tuple[Moon]) -> int:
    initial_moons = deepcopy(moons)
    cycles = list(-1 for _ in range(moons[0].dimensions))

    steps = 0
    while not all(v != -1 for v in cycles):
        steps += 1

        moons = simulate_step(moons)

        for i in range(len(cycles)):
            if cycles[i] != -1:
                continue

            if not has_cycle_at(moons, initial_moons, i):
                continue

            cycles[i] = steps

    return int(reduce(lcm, cycles))


def find_cycle_steps_by_brute_force(moons: List[Moon]) -> int:
    simulations = {tuple(moons)}
    steps = 1
    while (moons := simulate_step(moons)) not in simulations:
        steps += 1
    return steps


def main():
    file_path = Path(argv[1])

    moons = list()
    with file_path.open() as file:
        while text := file.readline().strip():
            matched = re.match(r'<x=(-?\d+), y=(-?\d+), z=(-?\d+)>', text)
            position = tuple(int(group) for group in matched.groups())
            moon = Moon(position)
            moons.append(moon)
    moons = tuple(moons)

    result = find_cycle_steps(moons)
    print(result)
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
