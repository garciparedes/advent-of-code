from collections import Counter
from pathlib import Path
from sys import argv
from typing import Tuple


def count_letters(text: str) -> Tuple[bool, bool]:
    counter = Counter(text)
    two, three = False, False
    for key, value in counter.most_common():
        if value > 3:
            continue
        elif value == 3:
            three = True
        elif value == 2:
            two = True
        else:
            break
    return two, three


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        twos, threes = 0, 0
        while text := file.readline().strip():
            two, three = count_letters(text)
            twos += two
            threes += three
        result = twos * threes

    print(result)


if __name__ == '__main__':
    main()
