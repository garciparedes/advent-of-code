from math import floor
from pathlib import Path
from sys import argv


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        result = 0
        while text := file.readline().strip():
            number = floor(int(text) / 3) - 2
            result += number

    print(result)


if __name__ == '__main__':
    main()
