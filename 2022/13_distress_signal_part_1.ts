import { readFileSync } from 'fs';

type Input = Pair[];

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
    return raw.trim().split('\n\n').map((rawBlock) => {
        const [rawOne, rawTwo] = rawBlock.trim().split('\n', 2);

        const one: PairEntry = eval(rawOne);
        const two: PairEntry = eval(rawTwo);

        return [one, two];
    });
}

function solve(input: Input): Output {
    return input.reduce((cum, curr, index) => {
        const condition = isInTheRightOrder(...curr);
        if (condition=== undefined) {
            throw new Error('It is not possible to determine if is in The Right Order');
        }
        return condition ? cum + (index + 1) : cum;
    }, 0)
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
