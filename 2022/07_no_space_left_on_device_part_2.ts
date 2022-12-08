import { readFileSync } from 'fs';

type Input = Command[];

type Command = CdCommand | LsCommand;

interface CdCommand { 
    kind: 'cd'; 
    argument: string; 
};

interface LsCommand { 
    kind: 'ls'; 
    output: LsCommandOutput;
};

type LsCommandOutput = LsCommandOutputEntry[];
type LsCommandOutputEntry = LsCommandOutputEntryDirectory | LsCommandOutputEntryFile;

interface LsCommandOutputEntryDirectory { 
    name: string; 
    size?: undefined;
}

interface LsCommandOutputEntryFile { 
    name: string; 
    size: number;
};

type Output = number;

type TreeNodeMap= Map<string, TreeNode>;

interface TreeNode { 
    size?: number; 
    parent?: TreeNode;
    children?: TreeNodeMap;
};

function main() {
    const input = readInput();
    const output = solve(input);
    writeOutput(output);
}

function readInput(): Input {
    const raw = readFileSync(0).toString();
    const rawCommands = raw.trim().split('$');
    rawCommands.shift();

    const commands = rawCommands.map(parseCommand)

    return commands;
}

function parseCommand(rawCommand: string): Command {
    const rawLines = rawCommand.trim().split('\n');
    const rawFirstLine = rawLines.shift();

    if (!rawFirstLine) {
        throw new Error('Empty command!');
    }

    const firstLine = rawFirstLine.trim();

    const [kind, argument] = firstLine.split(' ', 2);

    if (kind === 'cd') {
        return {kind, argument};
    }

    if (kind !== 'ls') {
        throw new Error('Unrecognized command!');
    }

    const output = rawLines.map(parseOutputLine);

    return { kind, output };
}

function parseOutputLine(rawLine: string): LsCommandOutputEntry {
    const [first, name] = rawLine.trim().split(' ');
    if (first === 'dir') {
        return { name };
    }
    return { name, size: Number(first) };
}

function solve(input: Input): Output {
    const root = computeNodes(input);
    computeSize(root);
    const required = computeRequired(root);
    return findRemovableSize(root, required);
}

function computeNodes(input: Input): TreeNode {
    const root: TreeNode = { children: new Map() };

    let current = root;
    for (const command of input) {
        const { kind } = command;
        if (kind === 'cd') {
            current = moveTo(current, command, root);
        } else {
            addChildren(current, command);
        }
    }

    return root;
}


function moveTo(current: TreeNode, command: CdCommand, root: TreeNode): TreeNode {
    const { argument } = command;
    let next;
    if (argument === '..') {
        next = current.parent;
    } else if (argument == '/') {
        next = root;
    } else {
        next = current.children?.get(argument);
    }
    if (!next) {
        throw new Error('No target node!');
    }
    return next;
}

function addChildren(current: TreeNode, command: LsCommand): void {
    const { children: nodes } = current;
    if (!nodes) {
        throw new Error('Current node is not a directory!');
    }

    const { output } = command;
    for (const entry of output) {
        const { name, size } = entry;

        if (nodes.has(name)) {
            console.log(nodes.get(name))
            continue;
        }

        let children;
        if (!size) {
            children = new Map();
        }

        const child = { parent: current, children, size };

        nodes.set(name, child);
    }
}


function computeSize(current: TreeNode): number {
    let { size, children } = current;
    if (size) {
        return size;
    }

    if (!children) {
        throw new Error('Node without children!');
    }

    size = [...children.values()].reduce((cum, child) => {
        const actual = computeSize(child);
        return cum + actual;
    }, 0);

    current.size = size;

    return size;
}

function computeRequired(current: TreeNode): number {
    return 30000000 - (70000000 - (current.size ?? 0));
}

function findRemovableSize(current: TreeNode, required: number): number {
    const { size } = current;
    if (!size) {
        throw new Error('Current does not have size!');
    }

    let bestSize = size; 

    const { children } = current;
    if (!children) {
        throw new Error('Current does not have children!');
    }

    for (const child of children.values()) {
        if (!child.children) {
            continue;
        }

        const childSize = child.size ?? 0;

        if (childSize >= required) {
            const bestChildSize = findRemovableSize(child, required);
            if (bestChildSize < bestSize) {
                bestSize = bestChildSize;
            }
        }
        
    }
    
    return bestSize;

}

function isSummable(current: TreeNode): boolean {
    const { children, size } = current;
    if (!children) {
        return false;
    }
    return !size || size <= 100000;
}

function writeOutput(output: Output): void {
    const raw = output;
    console.log(raw);
}

main();
