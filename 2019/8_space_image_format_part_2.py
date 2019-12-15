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
    numbers = map(int, text)

    image = list()
    for pixel in zip(*batch(numbers, WIDTH * HEIGHT)):
        for dimension in pixel:
            if dimension == 2:
                continue
            image.append(dimension)
            break

    image = list(batch(image, WIDTH))

    for row in image:
        print(''.join('#' if pixel == 1 else ' ' for pixel in row))


if __name__ == '__main__':
    main()
