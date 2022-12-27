import { readFileSync } from 'fs';

type Input = PairEntry[];

type PairEntry = number | PairEntry[];
type Pair = [PairEntry, PairEntry];

type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    return raw
        .trim()
        .replace(/\n\n/g, '\n')
        .split('\n')
        .map((rawLine) => eval(rawLine.trim()));
}

function solve(input: Input): Output {
    const [a, b] = [[[2]], [[6]]]; 
    input.push(a, b);
    input.sort((a, b) => {
        const condition = isInTheRightOrder(a, b);
        if (condition === true) {
            return -1;
        } else if (condition === undefined) {
            return 0;
        } else {
            return 1;
        }
    });
    return (input.indexOf(a) + 1) * (input.indexOf(b) + 1);
}

function isInTheRightOrder(left: PairEntry, right: PairEntry): boolean | undefined {
    if (left instanceof Array && right instanceof Array) {
        for (let i = 0; i < left.length && i < right.length; i += 1) {
            const curr = isInTheRightOrder(left[i], right[i]);
            if (curr !== undefined) {
                return curr
            }
        }
        if (left.length === right.length) {
            return undefined;
        }
        return left.length < right.length; 
    }

    if (isNumber(left) !== isNumber(right)) {
        if (isNumber(left)) {
            return isInTheRightOrder([left], right)
        } else {
            return isInTheRightOrder(left, [right])
        }
    }
    
    if (left === right) {
        return undefined;
    }
    return left < right;
}

function isNumber(value: unknown): boolean {
    return typeof value === 'number';
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
