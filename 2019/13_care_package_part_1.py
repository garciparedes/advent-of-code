from collections import defaultdict, namedtuple
from pathlib import Path
from sys import argv
from typing import List, Tuple, Dict
import itertools as it

from intcode import IntCode

TILE_KINDS = {
    'EMPTY': 0,
    'WALL': 1,
    'BLOCK': 2,
    'PADDLE': 3,
    'BALL': 4,
}

Tile = namedtuple('Tile', {'x': int, 'y': int, 'kind': int})


def grouper(n, iterable):
    """ From 'https://stackoverflow.com/a/1625013/3921457' """
    args = [iter(iterable)] * n
    return it.zip_longest(*args)


def simulate_game_tiles(program: List[int]) -> List[Tile]:
    machine = IntCode(program)
    output = machine.execute()

    tiles = [Tile(x, y, tile_id) for x, y, tile_id in grouper(3, output)]

    return tiles


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    board = simulate_game_tiles(program)
    result = sum(1 for cell in board if cell.kind == TILE_KINDS['BLOCK'])
    print(result)


if __name__ == '__main__':
    main()
