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
    const max = 4_000_000;
    // const max = 20;
    for (let y = 0; y <= max; y += 1) {
        const beacon = solveY(y, 0, max, input);
        if (beacon) {
            return beacon.x * 4_000_000 + beacon.y;
        }
    }
    throw new Error('Beacon not found!');
}

function solveY(y: number, x0: number, xn: number, sensors: Sensor[]): Coordinates | undefined {
    if (x0 >= xn) {
        return undefined;
    }

    if (sensors.length === 0) {
        if (xn - x0 !== 1) {
            return undefined;
        }
        return { y, x: xn };
    }

    const [current, ...others] = sensors;
    const { location, closestBeacon } = current;

    const distance = computeDistance(location, closestBeacon);

    if (!(location.y - distance <= y && y <= location.y + distance)) {
        return solveY(y, x0, xn, others);
    }

    const radio = distance - Math.abs(location.y - y);
    const [left, right] = [location.x - radio, location.x + radio - 1];
    const [leftXn, rightX0] = [Math.min(xn, left - 1), Math.max(x0, right + 1)];

    return solveY(y, x0, leftXn, others) ?? solveY(y, rightX0, xn, others);
}




function computeDistance(a: Coordinates, b: Coordinates): number {
    return Math.abs(a.x - b.x) + Math.abs(a.y - b.y);
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
