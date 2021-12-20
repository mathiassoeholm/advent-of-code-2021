export function parseInput(input: string) {
  const [decoder, image] = input.split("\n\n");
  return {
    decoder: decoder.split("").filter((c) => c !== "\n"),
    image: image.split("\n"),
  };
}

export function padImage(image: string[]) {
  return [
    ".".repeat(image.length + 4),
    ".".repeat(image.length + 4),
    ...image.map((row) => `..${row}..`),
    ".".repeat(image.length + 4),
    ".".repeat(image.length + 4),
  ];
}

export function selectSorrounding(image: string[], x: number, y: number) {
  return [
    [x - 1, y - 1],
    [x, y - 1],
    [x + 1, y - 1],
    [x - 1, y],
    [x, y],
    [x + 1, y],
    [x - 1, y + 1],
    [x, y + 1],
    [x + 1, y + 1],
  ]
    .map(([x1, y1]) => image[y1]?.[x1] ?? ".")
    .join("");
}

export function convertPixelsToNumber(pixels: string) {
  const binaryString = pixels
    .split("")
    .map((pixel) => (pixel === "#" ? "1" : "0"))
    .join("");
  return parseInt(binaryString, 2);
}

export function solveChallenge(input: string) {
  let { decoder, image } = parseInput(input);
  for (let i = 0; i < 2; i++) {
    image = padImage(image);
    image = decode(image, decoder);
    printImage(image);
  }
}

export function decode(image: string[], decoder: string[]) {
  let result = image.map(() => "");
  for (let y = 0; y < image.length; y++) {
    for (let x = 0; x < image[y].length; x++) {
      const sorrounding = selectSorrounding(image, x, y);
      const num = convertPixelsToNumber(sorrounding);
      const symbol = decoder[num];
      result[y] += symbol;
    }
  }

  return result;
}

export function printImage(image: string[]) {
  console.log(image.join("\n"));
}
