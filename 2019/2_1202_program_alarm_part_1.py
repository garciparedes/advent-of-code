from pathlib import Path
from sys import argv
from typing import List


def restore_alarm(program: List[int]) -> List[int]:
    program[1] = 12
    program[2] = 2
    return program


def execute_program(program: List[int]) -> List[int]:
    idx = -1
    while (idx := idx + 1) < len(program):
        op = program[idx]
        if op == 99:
            break
        a_idx = program[(idx := idx + 1)]
        b_idx = program[(idx := idx + 1)]
        dest_idx = program[(idx := idx + 1)]

        if op == 1:
            program[dest_idx] = program[a_idx] + program[b_idx]
        elif op == 2:
            program[dest_idx] = program[a_idx] * program[b_idx]

    return program


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    restore_alarm(program)
    execute_program(program)

    result = program[0]
    print(result)
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
