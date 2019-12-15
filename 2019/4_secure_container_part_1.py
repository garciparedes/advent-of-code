from pathlib import Path
from sys import argv


def check_valid(current: int) -> bool:
    duplicated = False
    current_str = str(current)
    for i in range(5):
        if current_str[i] == current_str[i + 1]:
            duplicated |= True
        elif not current_str[i] <= current_str[i + 1]:
            return False
    return duplicated


def password_cardinality(minimum: int, maximum: int) -> int:
    combinations = 0
    current = minimum

    while current <= maximum:
        if check_valid(current):
            combinations += 1
        current += 1

    return combinations


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()
    minimum, maximum = map(int, text.split('-'))

    result = password_cardinality(minimum, maximum)
    print(result)


if __name__ == '__main__':
    main()
