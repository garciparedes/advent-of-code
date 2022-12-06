import { readFileSync } from 'fs';

type InputCase = [string, string];
type Input = InputCase[];
type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    const input = raw.trim().split('\n').map((raw) => {
        const line = raw.trim();
        return [line.slice(0, line.length / 2), line.slice(line.length / 2)] as InputCase;
    })
    return input;
}

function solve(input: Input): Output {
    const priorities = input.map((rucksack) => {
        const left = new Set(rucksack[0]);
        const right = new Set(rucksack[1]);
        for (const rightItem of right) {
            if (left.has(rightItem)) {
                return computeScore(rightItem);
            }
        }
        throw new Error('One item must be in two compartments!')
    });
    return priorities.reduce((a, b) => a + b);
}

function computeScore(item: string): number {
    const ascii = item.charCodeAt(0);
    if (ascii > 96) {
        return ascii - 96;
    } else {
        return 26 + ascii - 64;
    }
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
