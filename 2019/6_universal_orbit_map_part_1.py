import re
from collections import defaultdict
from functools import reduce
from operator import or_
from pathlib import Path
from sys import argv
from typing import Set, Dict


def compute_orbits(graph: Dict[str, Set[str]], node: str) -> int:
    if node == 'COM':
        return 0
    for parent, children in graph.items():
        if node in children:
            return 1 + compute_orbits(graph, parent)


def main():
    file_path = Path(argv[1])

    graph = defaultdict(set)
    with file_path.open() as file:
        while text := file.readline().strip():
            match = re.match(r'(.+)\)(.+)', text)
            origin, destination = match.groups()
            graph[origin].add(destination)

    nodes = reduce(or_, graph.values())
    result = sum(compute_orbits(graph, node) for node in nodes)

    print(result)



if __name__ == '__main__':
    main()
