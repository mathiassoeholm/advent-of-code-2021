export function parseInput(input: string) {
  const [decoder, image] = input.split("\n\n");
  return {
    decoder: decoder.split("").filter((c) => c !== "\n"),
    image: image.split("\n"),
  };
}

export function padImage(image: string[], size = 2, padChar = ".") {
  return [
    ...Array(size).fill(padChar.repeat(image.length + size * 2)),
    ...image.map(
      (row) => `${padChar.repeat(size)}${row}${padChar.repeat(size)}`
    ),
    ...Array(size).fill(padChar.repeat(image.length + size * 2)),
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

export function solveChallenge(input: string, iterations: number) {
  let { decoder, image } = parseInput(input);
  let padChar = ".";
  for (let i = 0; i < iterations; i++) {
    const paddingSize = 2;
    image = padImage(image, paddingSize, padChar);
    image = decode(image, decoder);
    padChar = image[0][0];
  }

  const litPixels = image
    .join("")
    .split("")
    .filter((c) => c === "#").length;
  return litPixels;
}

export function decode(image: string[], decoder: string[]) {
  let result = image.map(() => "");
  for (let y = 0; y < image.length; y++) {
    for (let x = 0; x < image[y].length; x++) {
      const isAtEdge =
        x === 0 ||
        y === 0 ||
        x === image[y].length - 1 ||
        y === image.length - 1;
      const sorrounding = isAtEdge
        ? image[1][1].repeat(9)
        : selectSorrounding(image, x, y);
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
