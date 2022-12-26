import { readFileSync } from 'fs';

type Input = MonkeyState[];

interface MonkeyState {
    items: number[];
    operation: (item: number) => number;
    test: (item: number) => boolean;
    testTrue: number;
    testFalse: number;
    inspectedItems: number;
}

type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();

    const rawBlocks = raw
        .trim()
        .split('\n\n').map((rawBlock) => rawBlock.trim().split('\n').map((value) => value.trim()));

    const modBase = rawBlocks
        .map((rawBlock) => Number(rawBlock[3].split(' ').at(-1)!))
        .reduce((cum, curr) => cum * curr);

    return rawBlocks.map((rawBlock) => {
        const [, rawItems, rawOperation, rawTest, rawTestTrue, rawTestFalse] = rawBlock;

        const items = rawItems.split(': ', 2)[1].split(', ').map(Number);

        const operationString = rawOperation.split('= ')[1];
        const operation = (old: number) => eval(`(${operationString})`) % modBase;

        const testValue = Number(rawTest.split(' ').at(-1)!);
        const test = (item: number) => (item % testValue) === 0;

        const testTrue = Number(rawTestTrue.split(' ').at(-1)!);

        const testFalse = Number(rawTestFalse.split(' ').at(-1)!);

        return { items, operation, test, testTrue, testFalse, inspectedItems: 0 };
    })
}

function solve(input: Input): Output {
    const rounds = 10000;

    for (let i = 0; i < rounds; i += 1) {
        for (const monkeyState of input) {
            const { items, operation, test, testTrue, testFalse } = monkeyState;
            const { length: itemsLength } = items;
            for (let j = 0; j < itemsLength; j += 1) {
                let item = items[j];
                item = operation(item);
                if (test(item)) {
                    input[testTrue].items.push(item);
                } else {
                    input[testFalse].items.push(item);
                }
            }
            monkeyState.inspectedItems += itemsLength;
            monkeyState.items = [];
        }
    }
    const inspectedItems = input.map((item) => item.inspectedItems);
    inspectedItems.sort((a, b) => b - a);
    const [first, second] = inspectedItems;
    return first * second;
}


function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
