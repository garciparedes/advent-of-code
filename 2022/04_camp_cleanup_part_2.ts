import { readFileSync } from 'fs';

type Range = [number, number];
type InputItem = [Range, Range];
type Input = InputItem[];
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
        const [rawA, rawB] = line.split(',', 2);
        const a = rawA.split('-', 2).map(Number) as Range;
        const b = rawB.split('-', 2).map(Number) as Range;
        return [a, b];
    });
}

function solve(input: Input): Output {
    return input.reduce((count, item) => {
        if (checkItem(item)) {
            return count + 1;
        } else {
            return count
        }
    }, 0);
}

function checkItem(item: InputItem): boolean {
    return (
        (item[0][0] <= item[1][0] && item[1][0] <= item[0][1]) 
        || (item[0][0] <= item[1][1] && item[1][1] <= item[0][1])
        || (item[1][0] <= item[0][0] && item[0][0] <= item[1][1])
        || (item[1][0] <= item[0][1] && item[0][1] <= item[1][1])
    );
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
