from pathlib import Path
from sys import argv
from typing import List, Tuple
import itertools as it

from intcode import IntCode


def compute_signal(program, setting) -> int:
    second = 0
    for first in setting:
        machine = IntCode(program, [first, second])
        outputs = machine.execute()
        second = outputs[0]
    return second


def optimize_signals(program: List[int], minimum: int, maximum: int, count: int) -> int:
    return max(
        compute_signal(program, setting)
        for setting in it.permutations(range(minimum, maximum + 1), count)
    )


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    result = optimize_signals(program, 0, 4, 5)
    print(result)


if __name__ == '__main__':
    main()
