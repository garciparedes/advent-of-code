from collections import defaultdict
from typing import List, Tuple, Callable


class IntCodeException(Exception):
    pass


class InputException(IntCodeException):
    pass


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
        'RELATIVE_BASE': 9,
    }

    UNARY_OPS = {
        OPS['INPUT'],
        OPS['OUTPUT'],
        OPS['RELATIVE_BASE'],
    }
    BINARY_OPS = {
        OPS['JUMP_IF_TRUE'],
        OPS['JUMP_IF_FALSE'],
    }
    TERNARY_OPS = {
        OPS['SUM'],
        OPS['PRODUCT'],
        OPS['LESS_THAN'],
        OPS['EQUALS'],
    }

    MODE = {
        'POSITION': 0,
        'IMMEDIATE': 1,
        'RELATIVE': 2,
    }

    def __init__(self, program: List[int], inputs: List[int] = None, input_fn: Callable[['IntCode'], int] = None):
        if inputs is None:
            inputs = list()

        self.program = defaultdict(lambda: 0, dict(enumerate(program)))

        self.inputs_idx = 0
        self.inputs = list(inputs)
        self.input_fn = input_fn

        self.idx = 0
        self.relative_base_idx = 0

        self.outputs = list()

    def append_input(self, value: int) -> None:
        self.inputs.append(value)

    def next_input(self) -> int:
        if not self.inputs_idx < len(self.inputs):
            if self.input_fn is None:
                raise InputException()
            else:
                self.append_input(self.input_fn(self))
        result = self.inputs[self.inputs_idx]
        self.inputs_idx += 1
        return result

    @property
    def last_output(self) -> int:
        return self.outputs[-1]

    @property
    def halted(self) -> bool:
        return self.program[self.idx] == 99

    @property
    def executing(self):
        return self.idx != 0 and not self.halted

    def _get_position(self):
        self.idx += 1
        a_idx = self.program[self.idx]
        return a_idx

    def _get_value(self, mode: int, destination: bool = False) -> int:
        value = self._get_position()

        if mode == self.MODE['RELATIVE']:
            value += self.relative_base_idx

        if mode == self.MODE['IMMEDIATE'] or destination:
            return value
        elif mode in {self.MODE['POSITION'], self.MODE['RELATIVE']}:
            return self.program[value]
        else:
            raise ValueError(f'Unknown "{mode}" mode.')

    def execute_ternary_operation(self, op: int, a_mode: int, b_mode: int, dest_mode: int) -> None:
        a_value = self._get_value(a_mode)
        b_value = self._get_value(b_mode)
        dest_idx = self._get_value(dest_mode, True)

        try:
            if op == self.OPS['SUM']:
                self.program[dest_idx] = a_value + b_value
            elif op == self.OPS['PRODUCT']:
                self.program[dest_idx] = a_value * b_value
            elif op == self.OPS['LESS_THAN']:
                self.program[dest_idx] = int(a_value < b_value)
            elif op == self.OPS['EQUALS']:
                self.program[dest_idx] = int(a_value == b_value)
            else:
                raise ValueError(f'Unknown "{op}" operation.')
        except InputException as exc:
            self.idx -= 3
            raise exc

    def execute_binary_operation(self, op: int, input_mode, target_mode) -> None:
        input_value = self._get_value(input_mode)
        target_value = self._get_value(target_mode)

        try:
            if op == self.OPS['JUMP_IF_TRUE']:
                if input_value:
                    self.idx = target_value - 1
            elif op == self.OPS['JUMP_IF_FALSE']:
                if not input_value:
                    self.idx = target_value - 1
            else:
                raise ValueError(f'Unknown "{op}" operation.')
        except InputException as exc:
            self.idx -= 2
            raise exc

    def execute_unary_operation(self, op: int, target_mode) -> None:
        try:
            if op == self.OPS['INPUT']:
                target_idx = self._get_value(target_mode, True)
                self.program[target_idx] = self.next_input()
            elif op == self.OPS['OUTPUT']:
                target_value = self._get_value(target_mode)
                self.outputs.append(target_value)
            elif op == self.OPS['RELATIVE_BASE']:
                target_value = self._get_value(target_mode)
                self.relative_base_idx += target_value
            else:
                raise ValueError(f'Unknown "{op}" operation.')
        except InputException as exc:
            self.idx -= 1
            raise exc

    def parse_op(self) -> Tuple[int, int, int, int]:
        raw_op = self.program[self.idx]
        raw_op = str(raw_op).zfill(5)

        op = int(raw_op[-2:])
        a_mode = int(raw_op[-3])
        b_mode = int(raw_op[-4])
        dest_mode = int(raw_op[-5])
        return op, a_mode, b_mode, dest_mode

    def execute(self, **kwargs) -> List[int]:
        if self.halted:
            return self.outputs

        old_outputs_len = len(self.outputs)
        steps = 0
        while self.another_step(old_outputs_len=old_outputs_len, steps=(steps := steps + 1), **kwargs):
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
                raise ValueError(f'Unknown "{op}" operation.')

            self.idx += 1

        return self.outputs

    def another_step(self, old_outputs_len: int, steps: int = None, max_steps: int = None,
                     interactive: bool = False, **kwargs) -> bool:
        return (
                self.idx < len(self.program)
                and (old_outputs_len == len(self.outputs) or not interactive)
                and (max_steps is None or steps < max_steps)
        )
