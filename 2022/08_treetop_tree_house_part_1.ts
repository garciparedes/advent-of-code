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
    const visible = findVisibles(input)

    const ans = visible.reduce((cum, row) => {
        return cum + row.reduce((rowCum, cell) => {
            return rowCum + (cell ? 1 : 0);
        }, 0)
    }, 0)

    return ans;
}

function findVisibles(input: Input): boolean[][] {
    const [n, m] = [input.length, input[0].length];
    const visible = Array.from(Array(n), _ => Array(m).fill(false));

    for (let i = 0; i < n; i += 1) {
        visible[i][0] = true;

        let current = input[i][0];
        for (let j = 1; j < m - 1; j += 1) {
            current = setVisible(current, input, visible, i, j);
        }
    }

    for (let i = 0; i < n; i += 1) {
        visible[i][m - 1] = true;

        let current = input[i][m - 1];
        for (let j = m - 2; j > 0; j -= 1) {
            current = setVisible(current, input, visible, i, j);
        }
    }

    for (let j = 0; j < m; j += 1) {
        visible[0][j] = true;

        let current = input[0][j];
        for (let i = 1; i < n - 1; i += 1) {
            current = setVisible(current, input, visible, i, j);
        }
    }

    for (let j = 0; j < m; j += 1) {
        visible[n - 1][j] = true;

        let current = input[n - 1][j];
        for (let i = n - 2; i > 0; i -= 1) {
            current = setVisible(current, input, visible, i, j);
        }
    }

    return visible;
}

function setVisible(current: number, input: number[][], visible: boolean[][], i: number, j: number): number {
    if (input[i][j] > current) {
        visible[i][j] = true;
        return input[i][j];
    }
    return current;

}


function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
