import { readFileSync } from 'fs';

type Input = string;
type Output = number;

const DETECTING_SIZE = 14;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    return raw;
}

function solve(input: Input): Output {
    let i = DETECTING_SIZE - 1

    const window = input.slice(0, i).split('');

    for (; i < input.length; i += 1) {
        const character = input.at(i) as string;
        window.push(character);
        if (new Set(window).size === window.length) {
            break;
        }
        window.shift();
    }

    return i + 1;
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
