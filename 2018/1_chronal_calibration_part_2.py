from pathlib import Path
from sys import argv
from typing import TextIO


def read_line_loop(file: TextIO) -> int:
    if not (text := file.readline().strip()):
        file.seek(0)
        text = file.readline().strip()
    number = int(text)
    return number


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        current = 0
        old_states = {current}
        while (current := read_line_loop(file) + current) not in old_states:
            old_states.add(current)

    print(current)


if __name__ == '__main__':
    main()
