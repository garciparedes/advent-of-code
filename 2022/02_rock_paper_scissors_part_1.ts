import { readFileSync } from 'fs';

type InputCase = ['A' | 'B' | 'C', 'X' | 'Y' | 'Z'];
type Input = InputCase[];
type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    const cases = raw.trim().split('\n').map((raw) => {
        const line = raw.trim();
        return [line[0], line[2]] as InputCase;
    })
    return cases;
}

function solve(input: Input): Output {
    const scores: number[] = input.map((inputCase) => {
        let score = 0;
        if (inputCase[1] === 'X') {
            score += 1;
            if (inputCase[0] === 'C') {
                score += 6;
            } else if (inputCase[0] == 'A') {
                score += 3
            }
        } else if (inputCase[1] === 'Y') {
            score += 2;
            if (inputCase[0] === 'A') {
                score += 6;
            } else if (inputCase[0] == 'B') {
                score += 3
            }
        } else {
            score += 3;
            if (inputCase[0] === 'B') {
                score += 6;
            } else if (inputCase[0] == 'C') {
                score += 3
            }
        }
        return score;
    });
    const totalScore = scores.reduce((a, b) => a + b);
    return totalScore;
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
