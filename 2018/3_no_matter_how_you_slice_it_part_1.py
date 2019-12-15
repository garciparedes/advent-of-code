import re
from pathlib import Path
from sys import argv
from typing import List

SIZE = 1000
PATTERN = re.compile(r'#(\d+) @ (?P<x_offset>\d+),(?P<y_offset>\d+): (?P<x_len>\d+)x(?P<y_len>\d+)')


def update_plane(plane: List[List[int]], x_offset: int, y_offset: int, x_len: int, y_len: int) -> List[List[int]]:
    x_offset, y_offset, x_len, y_len = int(x_offset), int(y_offset), int(x_len), int(y_len)
    for i in range(x_offset, x_offset + x_len):
        for j in range(y_offset, y_offset + y_len):
            plane[i][j] += 1
    return plane


def main():
    file_path = Path(argv[1])

    plane = [[0 for _ in range(SIZE)] for _ in range(SIZE)]

    with file_path.open() as file:
        while text := file.readline().strip():
            matched = re.match(PATTERN, text)
            update_plane(plane, **matched.groupdict())

    result = sum(sum(cell > 1 for cell in row) for row in plane)
    print(result)


if __name__ == '__main__':
    main()
