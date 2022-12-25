import { readFileSync } from 'fs';

type Input = Operation[];

interface Operation {
    cycles: number;
    value?: number;
};

type Output = boolean[][];

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    return raw.trim().split('\n').map((rawLine) => {
        const line = rawLine.trim();
        const [rawType, rawValue] = line.split(' ', 2)
        if (rawType === 'noop') {
            return { cycles: 1 }
        } else {
            return { cycles: 2, value: Number(rawValue) };
        }
    });
}

function solve(input: Input): Output {
    const board = Array.from(Array(6), _ => Array(40).fill(false))
    let cycle = 0;
    let x = 1;
    for (const { cycles, value } of input) {
        for (let i = 0; i < cycles; i += 1) {
            if (x - 1 <= cycle % 40 && cycle % 40 <= x + 1) {
                board[Math.floor(cycle / 40)][cycle % 40] = true;
            }
            cycle += 1;
        }
        if (value) {
            x += value;
        }
    }
    return board;
}

function writeOutput(output: Output): void {
    for (const row of output) {
        const line = row.reduce((cum, value) => {
            return cum + (value ? '#' : '.');
        }, '');
        console.log(line);
    }
}

main();
