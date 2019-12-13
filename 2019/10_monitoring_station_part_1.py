from pathlib import Path
from sys import argv
from typing import List, Tuple


def board_to_points(board: List[List[bool]]) -> Tuple[Tuple[int, int], ...]:
    points = list()
    for i, row in enumerate(board):
        for j, cell in enumerate(row):
            if not cell:
                continue
            points.append((i, j))

    return tuple(points)


def check_if_between(current: Tuple[int, int], target: Tuple[int, int], point: Tuple[int, int]) -> bool:
    dxc = point[0] - current[0]
    dyc = point[1] - current[1]

    dxl = target[0] - current[0]
    dyl = target[1] - current[1]

    if dxc * dyl - dyc * dxl != 0:
        return False

    if abs(dxl) >= abs(dyl):
        if dxl > 0:
            return current[0] <= point[0] <= target[0]
        else:
            return target[0] <= point[0] <= current[0]

    else:
        if dyl > 0:
            return current[1] <= point[1] <= target[1]
        else:
            return target[1] <= point[1] <= current[1]


def check_if_detectable(current: Tuple[int, int], target: Tuple[int, int], points: List[Tuple[int, int]]) -> bool:
    for point in points:
        if current == point:
            continue
        if target == point:
            continue
        if check_if_between(current, target, point):
            return False
    return True


def compute_detections(current: Tuple[int, int], points: Tuple[Tuple[int, int], ...]) -> int:
    detections = 0

    points = sorted(points, key=lambda x: abs(x[0] - current[0]) + abs(x[1] - current[1]))

    for i, point in enumerate(points):
        if point == current:
            continue
        if not check_if_detectable(current, point, points[:i]):
            continue
        detections += 1

    return detections


def main():
    file_path = Path(argv[1])

    board = list()
    with file_path.open() as file:
        while text := file.readline().strip():
            row = [cell == '#' for cell in text]
            board.append(row)

    points = board_to_points(board)

    result = max(compute_detections(x, points) for x in points)

    print(result)
    with (file_path.parent / f'{file_path.stem}.output').open('w') as file:
        file.write(f'{result}')


if __name__ == '__main__':
    main()
