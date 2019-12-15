from pathlib import Path
from sys import argv
from typing import List
import itertools as it

from intcode import IntCode


def compute_signal(program, setting) -> int:
    machines = [IntCode(program) for _ in range(len(setting))]
    for first, machine in zip(setting, machines):
        machine.append_input(first)

    second = 0
    running = True
    while running:
        for machine in machines:
            machine.append_input(second)
            outputs = machine.execute(interactive=True)
            second = outputs[-1]
            if machine.executed:
                running = False
                continue
    return machines[-1].last_output


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

    # result = optimize_signals(program, 0, 4, 5)

    machine = IntCode(program, [1])
    outputs = machine.execute()
    print(outputs)


if __name__ == '__main__':
    main()
