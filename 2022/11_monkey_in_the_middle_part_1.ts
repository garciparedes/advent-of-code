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

    return raw.trim().split('\n\n').map((rawBlock) => {
        const [
            _, rawItems, rawOperation, rawTest, rawTestTrue, rawTestFalse
        ] = rawBlock.trim().split('\n').map((value) => value.trim());


        const items = rawItems.split(': ', 2)[1].split(', ').map(Number);

        const operationString = rawOperation.split('= ')[1];
        const operation = (old: number) => eval(operationString);

        const testValue = Number(rawTest.split(' ').at(-1)!);
        const test = (item: number) => (item % testValue) === 0;

        const testTrue = Number(rawTestTrue.split(' ').at(-1)!);

        const testFalse = Number(rawTestFalse.split(' ').at(-1)!);

        return { items, operation, test, testTrue, testFalse, inspectedItems: 0 };
    })
}

function solve(input: Input): Output {
    const rounds = 20;

    for (let i = 0; i < rounds; i += 1) {
        for (const monkeyState of input) {
            const { items, operation, test, testTrue, testFalse } = monkeyState;

            const { length: itemsLength } = items;
            for (let j = 0; j < itemsLength; j += 1) {
                let item = items[j];
                item = Math.floor(operation(item) / 3);
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
    const sortedInspectedItems = input.map((item) => item.inspectedItems);
    sortedInspectedItems.sort((a, b) => b - a);
    const [first, second] = sortedInspectedItems;
    return first * second;
}


function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
