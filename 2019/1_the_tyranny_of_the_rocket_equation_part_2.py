from math import floor
from pathlib import Path
from sys import argv


def compute_fuel(mass: int) -> int:
    fuel = floor(mass / 3) - 2
    if fuel <= 0:
        return 0
    return fuel + compute_fuel(fuel)


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        result = 0
        while text := file.readline().strip():
            number = compute_fuel(int(text))
            result += number

    print(result)



if __name__ == '__main__':
    main()
