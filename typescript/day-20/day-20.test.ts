import path from "path";
import fs from "fs";
import {
  convertPixelsToNumber,
  padImage,
  parseInput,
  selectSorrounding,
  solveChallenge,
} from "./day-20";

const exampleImage = ["#..#.", "#....", "##..#", "..#..", "..###"];

describe("day 20", () => {
  it("can parse input", () => {
    const parsed = parseInput("..#.#\n###.\n\n#.\n.#");
    expect(parsed).toEqual({
      decoder: [".", ".", "#", ".", "#", "#", "#", "#", "."],
      image: ["#.", ".#"],
    });
  });

  it("can pad image", () => {
    const padded = padImage(["##", "##"]);
    expect(padded).toEqual([
      "......",
      "......",
      "..##..",
      "..##..",
      "......",
      "......",
    ]);
  });

  it("can select sorrounding pixels", () => {
    let pixels = selectSorrounding(exampleImage, 2, 2);
    expect(pixels).toBe("...#...#.");

    pixels = selectSorrounding(exampleImage, 0, 0);
    expect(pixels).toBe("....#..#.");
  });

  it("can map pixels to a binary number and do a lookup", () => {
    const pixels = "...#...#.";
    const num = convertPixelsToNumber(pixels);
    expect(num).toBe(34);
  });

  it("works with example-input.txt", () => {
    const input = fs.readFileSync(
      path.join(__dirname, "example-input.txt"),
      "utf-8"
    );
    const result = solveChallenge(input, 50);
    expect(result).toBe(3351);
  });

  it("works with input.txt", () => {
    const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8");
    const result = solveChallenge(input, 50);
    console.log("Result is", result);
  });
});
