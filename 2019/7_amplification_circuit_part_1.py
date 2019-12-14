from pathlib import Path
from sys import argv
from typing import List, Tuple
import itertools as it


class IntCode(object):
    OPS = {
        'SUM': 1,
        'PRODUCT': 2,
        'INPUT': 3,
        'OUTPUT': 4,
        'JUMP_IF_TRUE': 5,
        'JUMP_IF_FALSE': 6,
        'LESS_THAN': 7,
        'EQUALS': 8,
    }

    UNARY_OPS = {OPS['INPUT'], OPS['OUTPUT']}
    BINARY_OPS = {OPS['JUMP_IF_TRUE'], OPS['JUMP_IF_FALSE']}
    TERNARY_OPS = {OPS['SUM'], OPS['PRODUCT'], OPS['LESS_THAN'], OPS['EQUALS']}

    MODE = {
        'POSITION': 0,
        'IMMEDIATE': 1,
    }

    def __init__(self, program: List[int], inputs: List[int]):
        self.program = list(program)
        self.inputs = list(inputs)

        self.idx = 0
        self.outputs = list()
        self.executed = False

    def _get_position(self):
        self.idx += 1
        a_idx = self.program[self.idx]
        return a_idx

    def _get_value(self, mode: int):
        idx = self._get_position()
        if mode == self.MODE['POSITION']:
            value = self.program[idx]
        elif mode == self.MODE['IMMEDIATE']:
            value = idx
        else:
            raise ValueError
        return value

    def execute_ternary_operation(self, op: int, a_mode: int, b_mode: int, dest_mode: int) -> None:
        a_value = self._get_value(a_mode)
        b_value = self._get_value(b_mode)

        dest_idx = self._get_position()
        if op == self.OPS['SUM']:
            self.program[dest_idx] = a_value + b_value
        elif op == self.OPS['PRODUCT']:
            self.program[dest_idx] = a_value * b_value
        elif op == self.OPS['LESS_THAN']:
            self.program[dest_idx] = a_value < b_value
        elif op == self.OPS['EQUALS']:
            self.program[dest_idx] = a_value == b_value
        else:
            raise ValueError

    def execute_binary_operation(self, op: int, input_mode, target_mode) -> None:
        input_value = self._get_value(input_mode)
        target_value = self._get_value(target_mode)

        if op == self.OPS['JUMP_IF_TRUE']:
            if input_value != 0:
                self.idx = target_value - 1
        elif op == self.OPS['JUMP_IF_FALSE']:
            if input_value == 0:
                self.idx = target_value - 1

    def execute_unary_operation(self, op: int, target_mode) -> None:
        if op == self.OPS['INPUT']:
            target_idx = self._get_position()
            self.program[target_idx] = self.inputs.pop(0)
        elif op == self.OPS['OUTPUT']:
            target_value = self._get_value(target_mode)
            self.outputs.append(target_value)

    def parse_op(self) -> Tuple[int, int, int, int]:
        raw_op = self.program[self.idx]
        raw_op = str(raw_op).zfill(5)

        op = int(raw_op[-2:])
        a_mode = int(raw_op[-3])
        b_mode = int(raw_op[-4])
        dest_mode = int(raw_op[-5])
        return op, a_mode, b_mode, dest_mode

    def execute(self) -> List[int]:
        if self.executed:
            return self.outputs

        self.idx = 0
        while self.idx < len(self.program):
            op, a_mode, b_mode, dest_mode = self.parse_op()

            if op == 99:
                break
            if op in self.TERNARY_OPS:
                self.execute_ternary_operation(op, a_mode, b_mode, dest_mode)
            elif op in self.BINARY_OPS:
                self.execute_binary_operation(op, a_mode, b_mode)
            elif op in self.UNARY_OPS:
                self.execute_unary_operation(op, a_mode)
            else:
                raise ValueError(op)

            self.idx += 1
        self.executed = True

        return self.outputs


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
