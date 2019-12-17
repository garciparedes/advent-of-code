from __future__ import annotations

import re
from collections import Counter
from math import ceil
from pathlib import Path
from sys import argv
from typing import Tuple, Dict

PATTERN = re.compile(r'(?P<antecedent>.+) => (?P<consequent>.+)')


def gcd(a, b):
    """Compute the greatest common divisor of a and b"""
    while b > 0:
        a, b = b, a % b
    return a


def lcm(a, b):
    """Compute the lowest common multiple of a and b"""
    return a * b / gcd(a, b)


class Ingredient(object):
    def __init__(self, name: str, amount: int):
        self.name = name
        self.amount = amount

    def __str__(self):
        return f'Ingredient({self.name},{self.amount})'

    def __repr__(self):
        return str(self)


class Cooker(object):

    def __init__(self, recipes: Dict[Ingredient, Tuple[Ingredient]], base: str):
        self.recipes = dict(recipes)
        self.base = base
        self.pantry = Counter()
        self.shopping_list = Counter()

    def get_recipe(self, name: str) -> Tuple[Ingredient, Tuple[Ingredient]]:
        consequent, antecedents = next(
            (consequent, antecedents)
            for consequent, antecedents in self.recipes.items()
            if consequent.name == name
        )
        return consequent, antecedents

    @property
    def recipes_with_base(self):
        return {
            consequent: antecedents
            for consequent, antecedents in self.recipes.items()
            if any(antecedent for antecedent in antecedents if antecedent.name == self.base)
        }

    def compute_needed(self, final: str, required: float = 1) -> None:
        consequent, antecedents, = self.get_recipe(final)

        minus = min(required, self.pantry[final])
        self.pantry[final] -= minus
        required -= minus

        remaining = ceil(required / consequent.amount) * consequent.amount - required
        self.pantry[final] += remaining

        real_required = ceil(required / consequent.amount)
        self.shopping_list[final] += real_required

        for antecedent in antecedents:
            if antecedent.name == self.base:
                continue
            self.compute_needed(
                antecedent.name,
                antecedent.amount * real_required,
            )

    @property
    def base_amount(self) -> int:
        assert all(value >= 0 for value in self.shopping_list.values())

        amount = 0
        for consequent, antecedents in self.recipes_with_base.items():
            assert len(antecedents) == 1
            antecedent = antecedents[0]
            amount += antecedent.amount * self.shopping_list[consequent.name]
        return amount


def main():
    file_path = Path(argv[1])

    recipes = dict()
    with file_path.open() as file:
        while text := file.readline().strip():
            matched = re.match(PATTERN, text)

            antecedents = matched.group('antecedent').split(', ')
            antecedents = map(lambda x: x.split(' '), antecedents)
            antecedents = map(lambda x: Ingredient(x[1], int(x[0])), antecedents)
            antecedents = tuple(antecedents)

            consequent = matched.group('consequent').split(' ')
            consequent = Ingredient(consequent[1], int(consequent[0]))

            recipes[consequent] = antecedents

    cooker = Cooker(recipes, 'ORE')
    cooker.compute_needed('FUEL')

    result = cooker.base_amount
    print(result)


if __name__ == '__main__':
    main()
