from pathlib import Path
from sys import argv
from typing import List

from intcode import IntCode


def restore_alarm(program: List[int]) -> List[int]:
    program[1] = 12
    program[2] = 2
    return program


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    restore_alarm(program)

    machine = IntCode(program)
    machine.execute()
    result = machine.program[0]

    print(result)
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
