from __future__ import annotations

import re
from copy import deepcopy
from pathlib import Path
from sys import argv
from typing import Tuple, Set
import itertools as it


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


def simulate_steps(moons: Set[Moon], steps: int) -> Set[Moon]:
    moons = deepcopy(moons)
    for _ in range(steps):
        for a, b in it.permutations(moons, 2):
            a.update_velocity(b)
        for moon in moons:
            moon.update_position()
    return moons


def main():
    file_path = Path(argv[1])

    moons = set()
    with file_path.open() as file:
        while text := file.readline().strip():
            matched = re.match(r'<x=(-?\d+), y=(-?\d+), z=(-?\d+)>', text)
            position = tuple(int(group) for group in matched.groups())
            moon = Moon(position)
            moons.add(moon)

    simulated_moons = simulate_steps(moons, 1000)
    result = sum(moon.energy for moon in simulated_moons)
    print(result)
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
