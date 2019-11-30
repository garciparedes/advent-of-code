from pathlib import Path
from sys import argv
from typing import Optional, Sequence


def check_box(a: str, b: str) -> Optional[str]:
    difference_idx = None
    for idx, (letter_a, letter_b) in enumerate(zip(a, b)):
        if letter_a == letter_b:
            continue
        if difference_idx is not None:
            return None
        difference_idx = idx
    result = a[:difference_idx] + a[difference_idx + 1:]
    return result


def find_box(data: Sequence[str]) -> Optional[str]:
    for i, a in enumerate(data):
        for b in data[i + 1:]:
            if result := check_box(a, b):
                return result
    return None


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        data = tuple(row.strip() for row in file.readlines())
        result = find_box(data)

    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
