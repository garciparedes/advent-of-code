from collections import defaultdict
from pathlib import Path
from sys import argv
from typing import List, Tuple, Set, Dict


def directions_to_coordinates(text: str) -> Dict[Tuple[int, int], List[int]]:
    current = (0, 0)
    idx = 0
    coordinates = defaultdict(list)
    for direction in text.split(','):
        axis = direction[0]
        steps = int(direction[1:])
        if axis == 'U':
            for _ in range(steps):
                current = (current[0] + 1, current[1])
                coordinates[current].append(idx := idx + 1)
        elif axis == 'D':
            for _ in range(steps):
                current = (current[0] - 1, current[1])
                coordinates[current].append(idx := idx + 1)
        elif axis == 'R':
            for _ in range(steps):
                current = (current[0], current[1] + 1)
                coordinates[current].append(idx := idx + 1)
        elif axis == 'L':
            for _ in range(steps):
                current = (current[0], current[1] - 1)
                coordinates[current].append(idx := idx + 1)
        else:
            raise ValueError
    return coordinates


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        a_text = file.readline().strip()
        b_text = file.readline().strip()
    a_coords = directions_to_coordinates(a_text)
    b_coords = directions_to_coordinates(b_text)

    crosses = set(a_coords.keys()).intersection(b_coords.keys())

    result = min(min(a_coords[cross]) + min(b_coords[cross]) for cross in crosses)

    print(result)
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
