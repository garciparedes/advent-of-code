from pathlib import Path
from sys import argv

from intcode import IntCode


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    program = [int(number) for number in text.split(',')]

    machine = IntCode(program, [5])
    output = machine.execute()
    print(output)


if __name__ == '__main__':
    main()
