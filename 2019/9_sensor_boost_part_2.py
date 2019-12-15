from pathlib import Path
from sys import argv
from intcode import IntCode


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    machine = IntCode(program, [2])

    outputs = machine.execute()
    print(outputs)


if __name__ == '__main__':
    main()
