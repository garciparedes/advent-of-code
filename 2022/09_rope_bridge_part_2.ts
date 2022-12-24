import { readFileSync } from 'fs';

type Input = Step[];
interface Step { 
    direction: Direction;
    offset: number;
}
type Direction = 'U' | 'D' | 'L' | 'R';
type Output = number;

interface Coordinates {
    x: number;
    y: number;
}

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();

    return raw.trim().split('\n').map((rawLine) => {
        const line = rawLine.trim();
        const [direction, rawOffset] = line.split(' ', 2);

        if (!['U', 'D', 'L', 'R'].includes(direction)) {
            throw new Error(`Unexpected direction: ${direction}`)
        } 

        return { direction: direction as Direction, offset: Number(rawOffset) };
    })
}

function solve(input: Input): Output {
    let rope: Coordinates[] = [...Array(10).keys()].map(() => ({x: 0, y: 0}));

    const tailCoordinates = new Set<string>();
    addTail(tailCoordinates, rope);

    for (const {direction, offset} of input) {
        for (let i = 0; i < offset; i += 1) {
            rope = updateRope(rope, direction);
            addTail(tailCoordinates, rope);
        }
    }
    return tailCoordinates.size;
}

function updateRope(old: Coordinates[], direction: Direction): Coordinates[] {
    const rope = [...old];
    rope[0] = updateHead(rope[0], direction);
    for (let j = 0; j < rope.length - 1; j += 1) {
        rope[j + 1] = updateTail(rope[j + 1], rope[j]);
    }
    return rope;
}

function updateHead(old: Coordinates, direction: Direction): Coordinates {
    const head = {...old};
    if (direction === 'U') {
        head.x += 1;
    } else if (direction === 'D') {
        head.x -= 1;
    } else if (direction === 'R') {
        head.y += 1
    } else {
        head.y -= 1;
    }
    return head;
}

function updateTail(old: Coordinates, head: Coordinates): Coordinates {
    const tail = {...old};
    const distance = Math.max(Math.abs(tail.x - head.x), Math.abs(tail.y - head.y));
    if (distance > 1) {
        if (tail.x < head.x) {
            tail.x += 1;
            if (tail.y < head.y) {
                tail.y += 1;
            } else if (tail.y > head.y) {
                tail.y -= 1;
            }
        } else if (tail.x > head.x) {
            tail.x -= 1;
            if (tail.y < head.y) {
                tail.y += 1;
            } else if (tail.y > head.y) {
                tail.y -= 1;
            }
        } else {
            if (tail.y < head.y) {
                tail.y += 1;
            } else if (tail.y > head.y) {
                tail.y -= 1;
            }
        }
    }
    return tail;
}

function addTail(tailPositions: Set<string>, rope: Coordinates[]) {
    addObject(tailPositions, rope[rope.length - 1]);
}

function addObject(set: Set<string>, value: object): void {
    set.add(JSON.stringify(value));
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
