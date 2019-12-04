from pathlib import Path
from sys import argv


def duplicated_condition(digits: str, i: int) -> bool:
    return (
            (digits[i] == digits[i + 1]) and
            (i == 0 or digits[i - 1] != digits[i]) and
            (i == 4 or digits[i + 2] != digits[i])
    )


def check_valid(number: int) -> bool:
    duplicated = False
    digits = str(number)
    for i in range(5):
        if not digits[i] <= digits[i + 1]:
            return False
        if duplicated_condition(digits, i):
            duplicated |= True
    return duplicated


def password_cardinality(minimum: int, maximum: int) -> int:
    return sum(1 for current in range(minimum, maximum + 1) if check_valid(current))


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    minimum, maximum = map(int, text.split('-'))

    result = password_cardinality(minimum, maximum)
    print(result)
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
