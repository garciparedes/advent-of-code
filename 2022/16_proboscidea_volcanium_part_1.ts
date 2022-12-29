import { readFileSync } from 'fs';

type Input = Map<string, Valve>;
interface Valve {
    rate: number;
    tunnels: Map<string, number>;
}



type Output = number;

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    return new Map( 
        raw.trim().split('\n').map((rawLine) => {
            const [rawDescription, rawTunnels] = rawLine.trim().split(';', 2);

            const [rawName, rawRate] = rawDescription.trim().split('=', 2);
            const name = rawName.trim().split(' ').at(1)!.trim();
            const rate = Number(rawRate.trim());
            
            const tunnels = new Map(
                rawTunnels
                    .trim()
                    .split(/valves?/, 2)
                    .at(1)!
                    .split(', ')
                    .map((rawTunnel) => [rawTunnel.trim(), 1])
          );

            return [name, { rate, tunnels }];
        })
    );
}

function solve(input: Input): Output {
    const valves = computeDistances(input);

    const time = 30;
    const start = 'AA'

    return dfs(valves, start, time + 1, new Set([start]));
}

function computeDistances(input: Map<string, Valve>): Map<string, Valve> {
    const valves: Map<string, Valve> = new Map();
    for (const key of input.keys()) {
        const tunnels: Map<string, number> = new Map();
        let distance = 0;
        const queue = [...input.get(key)!.tunnels.keys()];
        while (queue.length) {
            distance += 1;
            const { length } = queue;
            for (let i = 0; i < length; i +=1) {
                const next = queue.shift()!;
                if (next === key || tunnels.has(next)) {
                    continue;
                }
                tunnels.set(next, distance);
                for (const nextnext of input.get(next)!.tunnels.keys()) {
                    queue.push(nextnext)
                }
            }
        }
        valves.set(key, { rate: input.get(key)!.rate, tunnels });
    }

    const removable = [...valves.entries()]
        .filter(([name, { rate }]) => (name !== 'AA' && rate === 0))
        .map(([name]) => name);

    return new Map(
        [...valves.entries()]
            .filter(([name]) => !removable.includes(name))
            .map(([name, { rate, tunnels }]) => {
                return [
                    name, 
                    { 
                        rate, 
                        tunnels: new Map(
                            [...tunnels.entries()].filter(([next]) => !removable.includes(next))
                        ),
                    }
                ];
            })
    );
}

function dfs(valves: Map<string, Valve>, position: string, time: number, visited: Set<string>): number {
    if (time <= 0) {
        return 0;
    }

    const valve = valves.get(position)!;
    const { rate, tunnels } = valve;

    let best = 0;
    for (const [nextPosition, distance] of tunnels.entries()) {
        if (visited.has(nextPosition)) {
            continue;
        }
        visited.add(nextPosition);

        const nextValue = dfs(valves, nextPosition, (time - 1) - distance, visited);
        if (best < nextValue) {
            best = nextValue;
        }

        visited.delete(nextPosition);
    }

    return best + rate * (time - 1);
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
