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
            if (inputCase[0] === 'A') {
                score += 3;
            } else if (inputCase[0] === 'B') {
                score += 1; 
            } else {
                score += 2; 
            }
        } else if (inputCase[1] === 'Y') {
            score += 3;
            if (inputCase[0] === 'A') {
               score += 1;
            } else if (inputCase[0] === 'B') {
               score += 2;
            } else {
               score += 3;
            }
        } else {
            score += 6;
            if (inputCase[0] === 'A') {
               score += 2;
            } else if (inputCase[0] === 'B') {
               score += 3;
            } else {
               score += 1;
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
