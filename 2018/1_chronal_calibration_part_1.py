from pathlib import Path
from sys import argv


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        result = 0
        while text := file.readline().strip():
            result += int(text)

    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
