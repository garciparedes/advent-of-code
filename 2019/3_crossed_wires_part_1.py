from pathlib import Path
from sys import argv
from typing import List, Tuple, Set


def directions_to_coordinates(text: str) -> Set[Tuple[int, int]]:
    current = (0, 0)
    coordinates = set()
    for direction in text.split(','):
        axis = direction[0]
        steps = int(direction[1:])
        if axis == 'U':
            for _ in range(steps):
                current = (current[0] + 1, current[1])
                coordinates.add(current)
        elif axis == 'D':
            for _ in range(steps):
                current = (current[0] - 1, current[1])
                coordinates.add(current)
        elif axis == 'R':
            for _ in range(steps):
                current = (current[0], current[1] + 1)
                coordinates.add(current)
        elif axis == 'L':
            for _ in range(steps):
                current = (current[0], current[1] - 1)
                coordinates.add(current)
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

    crosses = a_coords.intersection(b_coords)

    result = min(abs(cross[0]) + abs(cross[1]) for cross in crosses)

    print(result)



if __name__ == '__main__':
    main()
