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

    return raw.trim().split('\n').map((rawLine) => {
        const line = rawLine.trim();

        const row = [];
        for (const c of line) {
            row.push(Number(c));
        }

        return row;
    });
}

function solve(input: Input): Output {
    const scores = computeScenicScores(input)

    const ans = scores.reduce((cum, row) => {
        return Math.max(
            cum, 
            row.reduce((rowCum, cell) => {
                return Math.max(rowCum, cell);
            }, 0)
        );
    }, 0);

    return ans;
}

function computeScenicScores(input: Input): number[][] {
    const [n, m] = [input.length, input[0].length];
    const score = Array.from(Array(n), _ => Array(m).fill(0));

    for (let i = 1; i < n - 1; i += 1) {
        for (let j = 1; j < m - 1; j += 1) {
            score[i][j] = computeScenicScore(input, i, j);
        }
    }

    return score;
}

function computeScenicScore(input: Input, i: number, j: number): number {
    const [n, m] = [input.length, input[0].length];
    const current = input[i][j];

    let score = 1;
    let offset;

    offset = 1;
    while (i + offset < n - 1 && input[i + offset][j] < current) {
        offset += 1;
    }
    score *= offset;

    offset = 1;
    while (i - offset > 0 && input[i - offset][j] < current) {
        offset += 1;
    }
    score *= offset;

    offset = 1;
    while (j + offset < m - 1 && input[i][j + offset] < current) {
        offset += 1;
    }
    score *= offset;

    offset = 1;
    while (j - offset > 0 && input[i][j - offset] < current) {
        offset += 1;
    }
    score *= offset;

    return score;
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
