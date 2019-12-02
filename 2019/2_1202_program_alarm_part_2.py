from pathlib import Path
from sys import argv
from typing import List, Tuple


def restore_alarm(program: List[int], noun: int = 12, verb: int = 2) -> List[int]:
    program[1] = verb
    program[2] = noun
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


def find_parameters(program: List[int], value: int) -> Tuple[int, int]:
    for noun in range(99):
        for verb in range(99):
            working_program = list(program)
            restore_alarm(working_program, noun, verb)
            execute_program(working_program)
            if working_program[0] == value:
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
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
