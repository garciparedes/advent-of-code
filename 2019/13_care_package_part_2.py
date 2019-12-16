import subprocess
from collections import defaultdict, namedtuple
from pathlib import Path
from random import choice
from sys import argv
from time import sleep
from typing import List, Tuple, Dict
import itertools as it

from intcode import IntCode, InputException

TILE_KINDS = {
    'EMPTY': 0,
    'WALL': 1,
    'BLOCK': 2,
    'PADDLE': 3,
    'BALL': 4,
}


def grouper(n, iterable):
    """ From 'https://stackoverflow.com/a/1625013/3921457' """
    args = [iter(iterable)] * n
    return it.zip_longest(*args)


def fn(m) -> int:
    board = defaultdict(lambda: 0, {(x, y): tile_id for x, y, tile_id in grouper(3, m.outputs)})
    print(tiles_to_str(board))

    ball = next(tile for tile, kind in board.items() if kind == TILE_KINDS['BALL'])
    paddle = next(tile for tile, kind in board.items() if kind == TILE_KINDS['PADDLE'])

    sleep(0.02)
    if ball[0] < paddle[0]:
        return -1
    elif ball[0] == paddle[0]:
        return 0
    elif ball[0] > paddle[0]:
        return 1
    else:
        raise Exception


def simulate_game_tiles(program: List[int]) -> int:
    machine = IntCode(program, input_fn=fn)

    machine.execute()
    board = defaultdict(lambda: 0, {(x, y): tile_id for x, y, tile_id in grouper(3, machine.outputs)})
    print(tiles_to_str(board))

    score_tile = next(kind for tile, kind in board.items() if tile[0] == -1 and tile[1] == 0)
    return score_tile


def trick_money(program: List[int]) -> List[int]:
    program = list(program)
    program[0] = 2
    return program


def tiles_to_str(board: Dict[Tuple[int, int], int]) -> str:
    subprocess.call("clear")

    minimum_x, maximum_x = min(cell[0] for cell in board.keys()), max(cell[0] for cell in board.keys())
    minimum_y, maximum_y = min(cell[1] for cell in board.keys()), max(cell[1] for cell in board.keys())

    paint = str()
    for j in range(minimum_y, maximum_y + 1):
        for i in range(minimum_x, maximum_x + 1):
            paint += f'{board[(i, j)]}' if board[(i, j)] else ' '
        paint += '\n'
    return paint


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    program = trick_money(program)
    result = simulate_game_tiles(program)
    print(result)


if __name__ == '__main__':
    main()
