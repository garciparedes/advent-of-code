import { readFileSync } from 'fs';

type Input = Operation[];

interface Operation {
    cycles: number;
    value?: number;
};

type Output = number;

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
    const observationCycles = new Set([
        20,
        60,
        100,
        140,
        180,
        220,
    ]);
    let cycle = 0;
    let x = 1;
    let strengthsSum = 0;

    for (const { cycles, value } of input) {
        for (let i = 0; i < cycles; i += 1) {
            cycle += 1;
            if (observationCycles.has(cycle)) {
                strengthsSum += cycle * x;
            }
        }
        if (value) {
            x += value;
        }
    }
    return strengthsSum;
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
