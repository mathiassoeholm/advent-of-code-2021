interface SnailNode {
  parent?: SnailNode;
  children: SnailNode[];
  value?: number;
}

let testSnailNumber = "[[[[0,7],4],[15,[0,13]]],[1,1]]";

const tree = parseSnailNumber(testSnailNumber);
const nodeToExplode = getPairToExplode(tree, 0);
const nodeToSplit = getNodeToSplit(tree);
const leftNode = findNodeInDirection(nodeToSplit!, "left");
const leftLeftNode = findNodeInDirection(leftNode!, "left");
const leftLeftLeftNode = findNodeInDirection(leftLeftNode!, "left");
const leftLeftLeftLeftNode = findNodeInDirection(leftLeftLeftNode!, "left");

console.log("Hello");

function findNodeInDirection(
  node: SnailNode,
  dir: "left" | "right",
  goingUp = true
): SnailNode | null {
  if (goingUp) {
    if (!node.parent) {
      return null;
    } else if (node.parent.children[0] !== node) {
      const childIndex = node.parent!.children.findIndex((c) => c === node);
      return findNodeInDirection(
        node.parent!.children[childIndex - 1],
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
      for (const child of [...node.children].reverse()) {
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
  const matches = testSnailNumber.matchAll(/(\[|\]|\d+|,)/g);
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
