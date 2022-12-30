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
    const board = computeBoard(input);
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

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
