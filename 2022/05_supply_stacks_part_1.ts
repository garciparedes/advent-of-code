import { readFileSync } from 'fs';

type Stack = string[];
type Move = { steps: number, from: string, to: string };
type Input = { stacks: Record<string, Stack>, moves: Move[] };
type Output = string;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();

    const [rawStacks, rawMoves] = raw.trimEnd().split('\n\n', 2).map((r) => r.trimEnd());
    const rawStacksLines = rawStacks.split('\n');

    const stacks: Record<string, Stack>  = {};
    for (let i = 0; i < (rawStacksLines.at(-1)?.length ?? 0); i += 1) {
        const rawId = rawStacksLines.at(-1)?.at(i)?.trim();
        if (!rawId) {
            continue;
        }
        const id = Number(rawId);
        const stack: Stack = [];
        for (let j = rawStacksLines.length - 2; j >= 0; j -= 1) {
            const rawCrate = rawStacksLines.at(j)?.at(i)?.trim();
            if (!rawCrate) {
                break;
            }
            stack.push(rawCrate);
        }
        stacks[id] = stack;
    }

    const moves = rawMoves.trim().split('\n').map((rawLine) => {
        const line = rawLine.trim();
        const words = line.split(' ');
        return { steps: Number(words[1]), from: words[3], to: words[5] };
    })

    return { stacks, moves };
}

function solve(input: Input): Output {
    const stacks = input.stacks;

    input.moves.forEach((move) => {
        for (let i = 0; i < move.steps; i += 1) {
            const item = stacks[move.from].pop()
            if (!item) {
                throw new Error('The stack is empty!')
            }
            stacks[move.to].push(item);
        }
    })
    
    let ans = '';
    for (const key in stacks) {
        ans += stacks[key].at(-1);
    }
    return ans;
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
