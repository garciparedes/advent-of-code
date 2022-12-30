import { readFileSync } from 'fs';

type Input = Coordinates[];

interface MinMaxCoordinates {
    min: Coordinates;
    max: Coordinates;
}

interface Coordinates {
    x: number;
    y: number;
    z: number;
}

type Board = boolean[][][];

type Output = number;


function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    return raw.trim().split('\n').map((rawLine) => {
        const [rawX, rawY, rawZ] = rawLine.trim().split(',', 3);
        const [x, y , z] = [Number(rawX), Number(rawY), Number(rawZ)];
        return { x, y, z };
    });
}

function solve(input: Input): Output {
    let board = computeBoard(input);
    board = recomputeBoard(board);
    return countSides(board);
}

function computeBoard(input: Input): Board {
    const n = 20 + 2;
    const board: Board = Array.from(
        Array(n), _ => Array.from(
            Array(n), _ => Array(n).fill(false)
        )
    );

    for (const { x, y, z } of input) {
        board[x + 1][y + 1][z + 1] = true;
    }
    return board;
}

function recomputeBoard(old: Board): Board {
    const n = old.length;

    const board: boolean[][][] = Array.from(
        Array(n), _ => Array.from(
            Array(n), _ => Array(n).fill(true)
        )
    );

    const memo: Set<string> = new Set(); 
    const queue: Coordinates[]= [];

    for (let i = 0; i < board.length; i += 1) {
        enqueue(queue, memo, 0, 0, i, n);
        enqueue(queue, memo, 0, i, 0, n);
        enqueue(queue, memo, 0, i, n - 1, n);
        enqueue(queue, memo, 0, n - 1, i, n);
        enqueue(queue, memo, i, 0, 0, n);
        enqueue(queue, memo, i, 0, n - 1, n);
        enqueue(queue, memo, i, n - 1, 0, n);
        enqueue(queue, memo, i, n - 1, n - 1, n);
        enqueue(queue, memo, n - 1, 0, i, n);
        enqueue(queue, memo, n - 1, i, 0, n);
        enqueue(queue, memo, n - 1, i, n - 1, n);
        enqueue(queue, memo, n - 1, n - 1, i, n);
    }

    while (queue.length) {
        const { x, y, z } = queue.shift()!;

        if (old[x][y][z] || !board[x][y][z]) {
            continue;
        }

        board[x][y][z] = false;

        enqueue(queue, memo, x - 1, y, z, n);
        enqueue(queue, memo, x + 1, y, z, n);
        enqueue(queue, memo, x, y - 1, z, n);
        enqueue(queue, memo, x, y + 1, z, n);
        enqueue(queue, memo, x, y, z - 1, n);
        enqueue(queue, memo, x, y, z + 1, n);
    }

    return board;
}

function countSides(board: Board): number {
    const n = board.length;
    let sides = 0;
    for (let i = 1; i < n - 1; i += 1) {
        for (let j = 1; j < n - 1; j += 1) {
            for (let k = 1; k < n - 1; k += 1) {
                if (board[i][j][k]) {
                    if (!board[i - 1][j][k]) {
                        sides += 1
                    }
                    if (!board[i + 1][j][k]) {
                        sides += 1
                    }
                    if (!board[i][j - 1][k]) {
                        sides += 1
                    }
                    if (!board[i][j + 1][k]) {
                        sides += 1
                    }
                    if (!board[i][j][k - 1]) {
                        sides += 1
                    }
                    if (!board[i][j][k + 1]) {
                        sides += 1
                    }
                }
            }
        }
    }
    return sides;
}


function enqueue(queue: Coordinates[], memo: Set<string>, x: number, y: number, z: number, n: number): void {
    const coordinates = { x, y, z };
    if (memo.has(JSON.stringify(coordinates))) {
        return;
    }
    memo.add(JSON.stringify(coordinates));
    if (!(0 <= x && x < n && 0 <= y && y < n && 0 <= z && z < n)) {
        return;
    }
    queue.push(coordinates);
}


function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
