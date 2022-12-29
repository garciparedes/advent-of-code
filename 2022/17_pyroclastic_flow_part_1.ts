import { readFileSync } from 'fs';

type Input = Operation[];

type Operation = '<' | '>'

type Output = number;

const ROCKS: Shape[] = [
    [{ x: 0, y: 0 }, { x: 0, y: 1 }, { x: 0, y: 2 }, { x: 0, y: 3 }],
    [{ x: 0, y: 1 }, { x: -1, y: 0 }, { x: -1, y: 1 }, { x: -1, y: 2 }, { x: -2, y: 1 }],
    [{ x: 0, y: 2 }, { x: -1, y: 2 }, { x: -2, y: 0 }, { x: -2, y: 1 }, { x: -2, y: 2 }],
    [{ x: 0, y: 0 }, { x: -1, y: 0 }, { x: -2, y: 0 }, { x: -3, y: 0 }],
    [{ x: 0, y: 0 }, { x: 0, y: 1 }, { x: -1, y: 0 }, { x: -1, y: 1 }],
]

type Shape = Coordinates[];

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
    return raw.trim().split('').map((rawChar) => {
        if (rawChar !== '<' && rawChar !== '>') {
            throw new Error('Invalid input!');
        }
        return rawChar;
    });
}

function solve(operations: Input): Output {
    const steps = 2022;

    const board: boolean[][] = []

    let operationIndex = 0;
    for (let i = 0; i < steps; i += 1) {
        const rockIndex = i % ROCKS.length;
        const rock = ROCKS[rockIndex];
        
        setupBoard(board, rock);
        const offset = { x: board.length - 1, y: 2 };

        while (true) {
            const operation = operations[operationIndex];
            if (operation === '<' && isValid(board, rock, { x: offset.x, y: offset.y - 1 })) {
                offset.y -= 1;
            } else if (operation === '>' && isValid(board, rock, { x: offset.x, y: offset.y + 1 })) {
                offset.y += 1;
            }
            operationIndex = (operationIndex + 1) % operations.length

            if (isValid(board, rock, { x: offset.x - 1, y: offset.y })) {
                offset.x -= 1;
            } else {
                commitRock(board, rock, offset);
                break;
            }

        }

    }

    // printBoard(board);

    return  getHighest(board);
}

function setupBoard(board: boolean[][], rock: Shape): void {
    const highest = getHighest(board);
    const additional = rock.reduce((cum, curr) => Math.max(cum, -curr.x), 0)
    while (!(board.length > highest + additional + 3)) {
        board.push(Array(7).fill(false))
    }
    while (!(board.length === 1 + highest + additional + 3)) {
        board.pop()
    }
}


function isValid(board: boolean[][], rock: Shape, offset: Coordinates): boolean {
    const [n, m] = [board.length, board[0].length];
    for (const point of rock) {
        const [x, y] = [point.x + offset.x, point.y + offset.y];
        if (!(0 <= x && x < n && 0 <= y && y < m) || board[x][y]) {
            return false;
        }
    }
    
    return true;
}

function commitRock(board: boolean[][], rock: Shape, offset: Coordinates) {
    for (const point of rock) {
        const [x, y] = [point.x + offset.x, point.y + offset.y];
        board[x][y] = true;
    }
}

function getHighest(board: boolean[][]): number {
    for (let i = board.length - 1; i >= 0; i -= 1) {
        if (board[i].some((cell) => cell)) {
            return i + 1;
        }
    }
    return board.length;
}


function printBoard(board: boolean[][]): void {
    for (let i = board.length - 1; i >= 0; i -= 1) {
        const draw = board[i].reduce((cum, curr) => cum + (curr ? '#' : '.'), '');
        console.log(`|${draw}|`);
    }
    console.log('+-------+')
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
