import { readFileSync } from 'fs';

type Input = number[][];
type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    const ans: number[][] = [];
    let current: number[] = [];
    raw.trim().split('\n').forEach((raw) => {
        const line = raw.trim();
        if (!line) {
            ans.push(current);
            current = [];
        } else {
            const value = Number(line);
            current.push(value);
        }
    });
    if (current) {
        ans.push(current);
    }
    return ans;
}

function solve(input: Input): Output {
    const sumCalories = input.map((elf) => elf.reduce(sum));
    const sortedCalories = sumCalories.sort();
    const ans = sortedCalories.slice(-3).reduce(sum);
    return ans;
}

function sum(a: number, b: number): number {
    return a + b;
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
