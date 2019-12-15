from pathlib import Path
from sys import argv
from typing import List, Tuple

from intcode import IntCode


def restore_alarm(program: List[int], noun: int = 12, verb: int = 2) -> List[int]:
    program[1] = verb
    program[2] = noun
    return program


def find_parameters(program: List[int], value: int) -> Tuple[int, int]:
    for noun in range(99):
        for verb in range(99):
            working_program = list(program)
            restore_alarm(working_program, noun, verb)

            machine = IntCode(working_program)
            machine.execute()

            if machine.program[0] == value:
                return verb, noun
    return -1, -1


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    noun, verb = find_parameters(program, 19690720)

    result = 100 * noun + verb
    print(result)


if __name__ == '__main__':
    main()
