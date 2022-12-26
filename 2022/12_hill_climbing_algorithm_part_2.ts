import { readFileSync } from 'fs';

type Input = { board: number[][], start: [number, number], end: [number, number] };
type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();

    let start: [number, number] | undefined = undefined;
    let end: [number, number] | undefined = undefined;
    const board = raw.trim().split('\n').map((rawLine, i) => {
        return rawLine.trim().split('').map((rawCell, j) => {
            if (rawCell === 'S') {
                start = [i, j];
                return 0;
            }

            if (rawCell === 'E') {
                end = [i, j];
                return 'z'.charCodeAt(0) - 'a'.charCodeAt(0);
            }

            return rawCell.charCodeAt(0) - 'a'.charCodeAt(0);
        })
    });
    if (start === undefined || end === undefined) {
        throw new Error('start and end must be defined!');
    }

    return { board, start, end };
}

function solve({ board, end }: Input): Output {
    const [n, m] = [board.length, board[0].length];
    const visited = Array.from(Array(n), _ => Array(m).fill(false));
    
    let steps = 0;
    
    const queue = [end];
    while (true) {
        const { length: levelLength  } = queue;
        if (!levelLength) {
            throw new Error('start not reached!');
        }
        for (let i = 0; i < levelLength; i += 1) {
            const [x, y] = queue.shift()!;

            if (board[x][y] === 0) {
                return steps;
            }
            
            if (x > 0 && !visited[x - 1][y]  && (board[x - 1][y] >= board[x][y] - 1)) {
                visited[x - 1][y] = true;
                queue.push([x - 1, y]);
            }
            if (x < n - 1 && !visited[x + 1][y]  && (board[x + 1][y] >= board[x][y] - 1)) {
                visited[x + 1][y] = true;
                queue.push([x + 1, y]);
            }
            if (y > 0 && !visited[x][y - 1]  && (board[x][y - 1] >= board[x][y] - 1)) {
                visited[x][y - 1] = true;
                queue.push([x, y - 1]);
            }
            if (y < m - 1 && !visited[x][y + 1]  && (board[x][y + 1] >= board[x][y] - 1)) {
                visited[x][y + 1] = true;
                queue.push([x, y + 1]);
            }
        }
        steps += 1
    }
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
