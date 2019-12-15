from collections import Counter
from pathlib import Path
from sys import argv
import itertools as it

WIDTH = 25
HEIGHT = 6


def batch(iterable, size):
    """From 'https://stackoverflow.com/a/59266741/3921457' """
    itr = iter(iterable)
    while item := tuple(it.islice(itr, size)):
        yield item


def main():
    file_path = Path(argv[1])

    with file_path.open() as file:
        text = file.readline().strip()

    counters = [Counter(frame) for frame in batch(text, WIDTH * HEIGHT)]
    counter = min(counters, key=lambda cnt: cnt['0'])
    result = counter['1'] * counter['2']

    print(result)


if __name__ == '__main__':
    main()
