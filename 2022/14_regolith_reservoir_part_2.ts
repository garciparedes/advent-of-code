import { readFileSync } from 'fs';

type Input = RockPath[];

type RockPath = Coordinates[];

interface Coordinates {
    x: number;
    y: number;
}

type MinMaxCoordinates = { min: Coordinates, max: Coordinates };

type Output = number;

type Board = Cell[][];
type Cell = -1 | 0 | 1 | 2 | 3;
type BoardGetter = (coordinates: Coordinates) => Cell;
type BoardSetter = (coordinates: Coordinates, value: Cell) => void;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    return raw.trim().split('\n').map(
        (rawLine) => rawLine.trim().split(' -> ').map(
            (rawCoordinates) => {
                const [rawX, rawY] = rawCoordinates.trim().split(',', 2);
                return { x: Number(rawX), y: Number(rawY) };
            }
        )
    );
}

function solve(input: Input): Output {
    const source = { x: 500, y: 0 };
    const { min, max } = findMinMaxCoordinates(input, source);
    const [n, m] = [max.y - min.y + 1, max.x - min.x + 1]

    const board: Board = Array.from(Array(n), _ => Array(m).fill(0));
    const getBoard = (coordinates: Coordinates): Cell => {
        if (!(min.x <= coordinates.x && coordinates.x <= max.x 
              && min.y <= coordinates.y && coordinates.y <= max.y)) {
            return -1;
        } 
        return board[coordinates.y - min.y][coordinates.x - min.x]
    };
    const setBoard = (coordinates: Coordinates, value: Cell) => {
        board[coordinates.y - min.y][coordinates.x - min.x] = value;
    }

    addSource(setBoard, source);
    addRockPaths(setBoard, input);
    addGround(setBoard, min, max);

    let count = 0;
    while (true) {
        if (!fillSand(setBoard, getBoard, source)) {
            break;
        }
        count += 1;
    }

    // printBoard(board);

    return count;
}

function findMinMaxCoordinates(paths: RockPath[], source: Coordinates): MinMaxCoordinates {
    const original = paths.reduce((cum, curr) => {
        const { min: minCurrCoordinates, max: maxCurrCoordinates } = curr.reduce(
            (pathCum, pathCurr) => ({
                min: {
                    x: Math.min(pathCum.min.x, pathCurr.x), 
                    y: Math.min(pathCum.min.y, pathCurr.y), 
                },
                max: {
                    x: Math.max(pathCum.max.x, pathCurr.x),
                    y: Math.max(pathCum.max.y, pathCurr.y),
                }
        }), { min: { ...source }, max: { ...source } });

        return {
            min: {
                x: Math.min(cum.min.x, minCurrCoordinates.x), 
                y: Math.min(cum.min.y, minCurrCoordinates.y), 
            },
            max: {
                x: Math.max(cum.max.x, maxCurrCoordinates.x),
                y: Math.max(cum.max.y, maxCurrCoordinates.y),
            }
        };
    }, { min: { ...source }, max: { ...source } });


    const rangeY = original.max.y - original.min.y;

    return { 
        min: { x: original.min.x - rangeY, y: original.min.y },
        max: { x: original.max.x + rangeY, y: original.max.y + 2 },
    }

}

function addSource(setBoard: BoardSetter, source: Coordinates): void {
    setBoard(source, 1);
}

function addRockPaths(setBoard: BoardSetter, paths: RockPath[]): void {
    for (const rockPath of paths) {
        let prev = rockPath.shift()!;
        for (const curr of rockPath) {
            if (prev.x === curr.x) {
                const [y0, yn] = [Math.min(prev.y, curr.y), Math.max(prev.y, curr.y)];
                for (let i = 0; i < (yn - y0) + 1; i += 1) {
                    setBoard({y: y0 + i, x: prev.x}, 2);
                }
            } else {
                const [x0, xn] = [Math.min(prev.x, curr.x), Math.max(prev.x, curr.x)];
                for (let i = 0; i < (xn - x0) + 1; i += 1) {
                    setBoard({y: prev.y, x: x0 + i }, 2);
                }
            }
            prev = curr;
        }
    }
}

function addGround(setBoard: BoardSetter, min: Coordinates, max: Coordinates): void {
    for (let i = 0; i < (max.x - min.x + 1); i += 1) {
        setBoard({x: min.x + i, y: max.y}, 2);
    }
}

function fillSand(setBoard: BoardSetter, getBoard: BoardGetter, source: Coordinates): boolean {
    const sand = { ...source };
    if (getBoard(source) !== 1) {
        return false;
    }

    let cell: Cell;
    let canMove = true;
    while (canMove) {
        canMove = false;

        cell = getBoard({ x: sand.x, y: sand.y + 1 })
        if (cell === -1) {
            return false;
        }
        if (cell === 0) {
            canMove = true;
            sand.y += 1
            continue;
        } 

        cell = getBoard({ x: sand.x - 1, y: sand.y + 1 })
        if (cell === -1) {
            return false;
        }
        if (cell === 0) {
            canMove = true;
            sand.x -= 1
            sand.y += 1
            continue;
        } 

        cell = getBoard({ x: sand.x + 1, y: sand.y + 1 })
        if (cell === -1) {
            return false;
        }
        if (cell === 0) {
            canMove = true;
            sand.x += 1
            sand.y += 1
            continue;
        }
    }


    setBoard(sand, 3);
    return true;
}

function printBoard(board: Board): void {
    const mapper: string[] = ['.', '+', '#', 'o']
    const draw = board.reduce<string>(
        (cum, curr) => cum + curr.reduce<string>((cum2, curr2) => `${cum2}${mapper[curr2]}`, '') + '\n', '');
    console.log(draw);
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
