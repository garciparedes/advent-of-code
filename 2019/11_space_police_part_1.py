from collections import defaultdict
from pathlib import Path
from sys import argv
from typing import List, Tuple, Dict

from intcode import IntCode

TURNS = {
    'COUNTERCLOCKWISE': 0,
    'CLOCKWISE': 1,
}


def update_current(old: Tuple[int, int], direction: str, turn) -> Tuple[Tuple[int, int], str]:
    if direction == 'UP':
        if turn == TURNS['CLOCKWISE']:
            return (old[0], old[1] + 1), 'RIGHT'
        elif turn == TURNS['COUNTERCLOCKWISE']:
            return (old[0], old[1] - 1), 'LEFT'
        else:
            raise ValueError(f'Turn "{turn}" is unknown.')
    elif direction == 'DOWN':
        if turn == TURNS['CLOCKWISE']:
            return (old[0], old[1] - 1), 'LEFT'
        elif turn == TURNS['COUNTERCLOCKWISE']:
            return (old[0], old[1] + 1), 'RIGHT'
        else:
            raise ValueError(f'Turn "{turn}" is unknown.')
    elif direction == 'LEFT':
        if turn == TURNS['CLOCKWISE']:
            return (old[0] + 1, old[1]), 'UP'
        elif turn == TURNS['COUNTERCLOCKWISE']:
            return (old[0] - 1, old[1]), 'DOWN'
        else:
            raise ValueError(f'Turn "{turn}" is unknown.')
    elif direction == 'RIGHT':
        if turn == TURNS['CLOCKWISE']:
            return (old[0] - 1, old[1]), 'DOWN'
        elif turn == TURNS['COUNTERCLOCKWISE']:
            return (old[0] + 1, old[1]), 'UP'
        else:
            raise ValueError(f'Turn "{turn}" is unknown.')
    else:
        raise ValueError(f'Direction "{direction}" is unknown.')


def simulate_painting_robot(program: List[int]) -> Dict[Tuple[int, int], int]:
    machine = IntCode(program)

    history = set()

    current = (0, 0)
    direction = 'UP'

    board = defaultdict(lambda: 0, {})

    while not machine.halted:
        history.add(current)

        machine.append_input(board[current])
        machine.execute(interactive=True)
        color = machine.last_output
        machine.execute(interactive=True)
        turn = machine.last_output

        board[current] = color

        current, direction = update_current(current, direction, turn)

    return board


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    board = simulate_painting_robot(program)
    result = len(board) - 1
    print(result)


if __name__ == '__main__':
    main()
