import { readFileSync } from 'fs';

type Input = Sensor[];

interface Sensor {
    location: Coordinates;
    closestBeacon: Coordinates;
}

interface Coordinates {
    x: number;
    y: number;
}

type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    return raw.trim().split('\n').map((rawLine) => {
        const [rawLocation, rawClosestBeacon] = rawLine.trim().split(':', 2);
        const location = parseCoordinates(rawLocation);
        const closestBeacon = parseCoordinates(rawClosestBeacon);

        return { location, closestBeacon };
    });
}

function parseCoordinates(raw: string): Coordinates {
    const [x, y] = raw
        .trim()
        .split('at', 2)
        .at(1)!
        .trim()
        .split(',', 2)
        .map((rawValue) => Number(rawValue.trim().split('=').at(1)!));

    return { x, y };
}

function solve(input: Input): Output {
    const y = 2000000;
    // const y = 20;
    const cannotContain = new Set();
    for (const { location, closestBeacon } of input) {
        const distance = computeDistance(location, closestBeacon);
        if (location.y - distance <= y && y <= location.y + distance) {
            const radio = distance - Math.abs(location.y - y);
            const [left, right] = [location.x - radio, location.x + radio - 1];

            for (let i = left; i < right; i += 1) {
                cannotContain.add(i);
            }
        }

    }
    return cannotContain.size;
}

function computeDistance(a: Coordinates, b: Coordinates): number {
    return Math.abs(a.x - b.x) + Math.abs(a.y - b.y);
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
