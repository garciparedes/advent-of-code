from pathlib import Path
from sys import argv
from typing import List, Tuple

OPS = {
    'SUM': 1,
    'PRODUCT': 2,
    'INPUT': 3,
    'OUTPUT': 4,
}

MODE = {
    'POSITION': 0,
    'IMMEDIATE': 1,
}


def execute_binary_operation(program: List[int], idx: int, op: int, a_mode: int, b_mode: int,
                             dest_mode: int) -> Tuple[List[int], int]:
    a_idx = program[(idx := idx + 1)]
    if a_mode == MODE['POSITION']:
        a_value = program[a_idx]
    elif a_mode == MODE['IMMEDIATE']:
        a_value = a_idx
    else:
        raise ValueError

    b_idx = program[(idx := idx + 1)]
    if b_mode == MODE['POSITION']:
        b_value = program[b_idx]
    elif b_mode == MODE['IMMEDIATE']:
        b_value = b_idx
    else:
        raise ValueError

    dest_idx = program[(idx := idx + 1)]
    if op == OPS['SUM']:
        program[dest_idx] = a_value + b_value
    elif op == OPS['PRODUCT']:
        program[dest_idx] = a_value * b_value
    return program, idx


def execute_unary_operation(program: List[int], idx: int, op: int, target_mode) -> Tuple[List[int], int]:
    target_idx = program[(idx := idx + 1)]
    if target_mode == MODE['POSITION']:
        target_value = program[target_idx]
    elif target_mode == MODE['IMMEDIATE']:
        target_value = target_idx
    else:
        raise ValueError

    if op == OPS['INPUT']:
        program[target_idx] = int(input())
    elif op == OPS['OUTPUT']:
        print(target_value, end=' ')

    return program, idx


def parse_op(raw_op: int) -> Tuple[int, int, int, int]:
    raw_op = str(raw_op).zfill(5)

    op = int(raw_op[-2:])
    a_mode = int(raw_op[-3])
    b_mode = int(raw_op[-4])
    dest_mode = int(raw_op[-5])
    return op, a_mode, b_mode, dest_mode


def execute_program(program: List[int]) -> List[int]:
    idx = -1
    while (idx := idx + 1) < len(program):
        op, a_mode, b_mode, dest_mode = parse_op(program[idx])

        if op == 99:
            break
        if op in (OPS['SUM'], OPS['PRODUCT']):
            program, idx = execute_binary_operation(program, idx, op, a_mode, b_mode, dest_mode)
        elif op in (OPS['INPUT'], OPS['OUTPUT']):
            program, idx = execute_unary_operation(program, idx, op, a_mode)
        else:
            raise ValueError(op)
    return program


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    execute_program(program)

    result = program[0]


if __name__ == '__main__':
    main()
