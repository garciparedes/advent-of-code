import { readFileSync } from 'fs';

type InputCase = [string, string, string];
type Input = InputCase[];
type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    const clean = raw.trim().split('\n').map((line) => line.trim());
    const input = [];
    for (let i = 0; i < clean.length; i += 3) {
        const chunk = clean.slice(i, i + 3) as InputCase;
        input.push(chunk)
    }
    return input;
}

function solve(input: Input): Output {
    const priorities = input.map((rucksack) => {
        const one = new Set(rucksack[0]);
        const two = new Set(rucksack[1]);
        const three = new Set(rucksack[2]);
        for (const oneItem of one) {
            if (two.has(oneItem) && three.has(oneItem)) {
                return computeScore(oneItem); 
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
