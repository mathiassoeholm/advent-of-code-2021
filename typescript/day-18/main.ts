import fs from "fs";
import path from "path";

interface SnailNode {
  parent?: SnailNode;
  children: SnailNode[];
  value?: number;
}

const input = fs.readFileSync(path.join(__dirname, "./input.txt"), "utf-8");

const numbers = input.split("\n");
let largestMagnitude = -Infinity;

for (let i = 0; i < numbers.length; i++) {
  for (let j = 0; j < numbers.length; j++) {
    if (i !== j) {
      const a = parseSnailNumber(numbers[i]);
      const b = parseSnailNumber(numbers[j]);
      const sum = addNumbers(a, b);
      const magnitude = getMagnitude(sum);

      largestMagnitude = Math.max(magnitude, largestMagnitude);
    }
  }
}

for (let i = 0; i < numbers.length; i++) {
  for (let j = 0; j < numbers.length; j++) {
    if (i !== j) {
      const a = parseSnailNumber(numbers[j]);
      const b = parseSnailNumber(numbers[i]);
      const sum = addNumbers(a, b);
      const magnitude = getMagnitude(sum);

      largestMagnitude = Math.max(magnitude, largestMagnitude);
    }
  }
}

console.log("Largest magnitude", largestMagnitude);

function getMagnitude(tree: SnailNode): number {
  if (tree.children.length === 0) {
    return tree.value!;
  }

  return (
    3 * getMagnitude(tree.children[0]) + 2 * getMagnitude(tree.children[1])
  );
}

function convertToString(tree: SnailNode): string {
  if (tree.value != undefined) {
    return tree.value.toString();
  }

  return "[" + tree.children.map(convertToString).join(",") + "]";
}

function addNumbers(aTree: SnailNode, bTree: SnailNode) {
  const sum: SnailNode = {
    children: [aTree, bTree],
  };
  aTree.parent = sum;
  bTree.parent = sum;

  let pairToExplode;
  let nodeToSplit;
  while (
    (pairToExplode = getPairToExplode(sum, 0)) !== null ||
    (nodeToSplit = getNodeToSplit(sum)) !== null
  ) {
    if (pairToExplode) {
      explodePair(pairToExplode);
    } else if (nodeToSplit) {
      splitNode(nodeToSplit);
    }
  }

  return sum;
}

function splitNode(node: SnailNode) {
  const left: SnailNode = {
    value: Math.floor(node.value! / 2),
    parent: node,
    children: [],
  };

  const right: SnailNode = {
    value: Math.ceil(node.value! / 2),
    parent: node,
    children: [],
  };

  node.children = [left, right];
  node.value = undefined;
}

function explodePair(node: SnailNode) {
  if (!nodeIsPair(node)) {
    throw new Error("Node is not a pair!");
  }

  const leftChild = node.children[0];
  const rightChild = node.children[1];

  const left = findNodeInDirection(node, "left");
  const right = findNodeInDirection(node, "right");

  if (left) {
    left.value! += leftChild.value!;
  }

  if (right) {
    right.value! += rightChild.value!;
  }

  node.value = 0;
  node.children = [];
}

function findNodeInDirection(
  node: SnailNode,
  dir: "left" | "right",
  goingUp = true
): SnailNode | null {
  if (goingUp) {
    if (!node.parent) {
      return null;
    } else if (
      (dir === "left" && node.parent.children[0] !== node) ||
      (dir === "right" &&
        node.parent.children[node.parent.children.length - 1] !== node)
    ) {
      const childIndex = node.parent!.children.findIndex((c) => c === node);
      return findNodeInDirection(
        node.parent!.children[dir === "left" ? childIndex - 1 : childIndex + 1],
        dir,
        false
      );
    } else {
      return findNodeInDirection(node.parent!, dir, true);
    }
  } else {
    if (node.value != undefined) {
      return node;
    } else {
      for (const child of dir === "left"
        ? [...node.children].reverse()
        : node.children) {
        const n = findNodeInDirection(child, dir, false);
        if (n) {
          return n;
        }
      }
    }
  }

  return null;
}

function getNodeToSplit(node: SnailNode): SnailNode | null {
  if (node.value && node.value >= 10) {
    return node;
  }

  for (const child of node.children) {
    const nodeToSplit = getNodeToSplit(child);
    if (nodeToSplit) {
      return nodeToSplit;
    }
  }

  return null;
}

function getPairToExplode(node: SnailNode, level: number): SnailNode | null {
  if (level === 4 && nodeIsPair(node)) {
    return node;
  } else if (node.children.length === 0) {
    return null;
  }

  for (const child of node.children) {
    const pairToExplode = getPairToExplode(child, level + 1);
    if (pairToExplode) {
      return pairToExplode;
    }
  }

  return null;
}

function nodeIsPair(node: SnailNode) {
  return (
    node.children.length === 2 &&
    node.children[0].children.length === 0 &&
    node.children[1].children.length === 0
  );
}

function parseSnailNumber(input: string) {
  const matches = input.matchAll(new RegExp("(\\[|\\]|\\d+|,)", "g"));
  let root: SnailNode | undefined;
  let currentNode: SnailNode | undefined;

  for (const match of matches) {
    switch (match[0]) {
      case "[": {
        const newNode: SnailNode = {
          parent: currentNode,
          children: [],
        };
        root = root ?? newNode;

        currentNode?.children.push(newNode);
        currentNode = newNode;

        break;
      }
      case "]":
        currentNode = currentNode?.parent;
        break;
      case ",": {
        break;
      }
      default: {
        const newNode: SnailNode = {
          parent: currentNode,
          children: [],
          value: parseInt(match[0]),
        };

        currentNode!.children.push(newNode);
      }
    }
  }

  return root!;
}
