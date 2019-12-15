import re
from collections import defaultdict
from pathlib import Path
from sys import argv
from typing import Set, Dict, List


def compute_orbits(graph: Dict[str, Set[str]], node: str) -> List[str]:
    if node == 'COM':
        return []
    for parent, children in graph.items():
        if node in children:
            return compute_orbits(graph, parent) + [parent]


def orbital_transfers(a: List[str], b: List[str]) -> int:
    a, b = list(a), list(b)
    while a[0] == b[0]:
        a.pop(0)
        b.pop(0)
    return len(a) + len(b)


def main():
    file_path = Path(argv[1])

    graph = defaultdict(set)
    with file_path.open() as file:
        while text := file.readline().strip():
            match = re.match(r'(.+)\)(.+)', text)
            origin, destination = match.groups()
            graph[origin].add(destination)

    you_orbits = compute_orbits(graph, 'YOU')
    san_orbits = compute_orbits(graph, 'SAN')

    while you_orbits[0] == san_orbits[0]:
        you_orbits.pop(0)
        san_orbits.pop(0)
    result = len(you_orbits) + len(san_orbits)

    print(result)



if __name__ == '__main__':
    main()
