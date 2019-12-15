from collections import defaultdict
from math import atan2
from pathlib import Path
from sys import argv
from typing import List, Tuple, Dict, Optional
import itertools as it


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


def check_if_obstacle(current: Tuple[int, int], target: Tuple[int, int],
                      points: List[Tuple[int, int]]) -> Optional[Tuple[int, int]]:
    for point in points:
        if current == point:
            continue
        if target == point:
            continue
        if check_if_between(current, target, point):
            return point
    return None


def compute_detections(current: Tuple[int, int], points: Tuple[Tuple[int, int], ...]) -> int:
    detections = 0

    points = sorted(points, key=lambda x: abs(x[0] - current[0]) + abs(x[1] - current[1]))

    for i, point in enumerate(points):
        if point == current:
            continue
        if check_if_obstacle(current, point, points[:i]):
            continue
        detections += 1

    return detections


def compute_groups(current: Tuple[int, int], points: Tuple[Tuple[int, int], ...]):
    points = sorted(points, key=lambda x: abs(x[0] - current[0]) + abs(x[1] - current[1]))

    groups = defaultdict(list)
    for i, point in enumerate(points):
        if point == current:
            continue
        obstacle = check_if_obstacle(current, point, points[:i])
        if obstacle is None:
            obstacle = point
        groups[obstacle].append(point)
    return groups


def clockwise_distance(base: Tuple[int, int], x: Tuple[int, int]) -> float:
    a, b = x[0] - base[0], x[1] - base[1]
    return atan2(b, a) - atan2(1, 0)


def iterate_groups(base: Tuple[int, int], groups: Dict[str, List[Tuple[int, int]]], idx: int) -> Tuple[int, int]:
    if sum(len(group) for group in groups.values()) < idx:
        raise ValueError

    firsts = groups.keys()
    firsts = sorted(firsts, key=lambda x: clockwise_distance(base, x), reverse=True)

    i = 0
    for key in it.cycle(firsts):
        if not any(groups[key]):
            continue
        i += 1
        current = groups[key].pop(0)
        if i == idx:
            return current


def main():
    file_path = Path(argv[1])

    board = list()
    with file_path.open() as file:
        while text := file.readline().strip():
            row = [cell == '#' for cell in text]
            board.append(row)

    points = board_to_points(board)

    base = max(points, key=lambda x: compute_detections(x, points))

    groups = compute_groups(base, points)
    final_point = iterate_groups(base, groups, 200)

    result = f'{final_point[1] * 100 + final_point[0]}'
    print(result)



if __name__ == '__main__':
    main()
