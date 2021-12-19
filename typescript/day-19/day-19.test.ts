import {
  getAllOrientations,
  getAllOrientationsForList,
  solveChallenge,
} from "./day-19";
import { exampleInput } from "./example-input";

import path from "path";
import fs from "fs";

describe("day 19", () => {
  it("can get different orientations for a single position", () => {
    expect(getAllOrientations(5, 6, -4)).toEqual(
      expect.arrayContaining([
        [5, 6, -4],
        [-5, 4, -6],
        [4, 6, 5],
        [-4, -6, 5],
        [-6, -4, -5],
      ])
    );
  });

  it("can get different orientations of a list of positions", () => {
    expect(
      getAllOrientationsForList([
        [-1, -1, 1],
        [-2, -2, 2],
      ])
    ).toEqual(
      expect.arrayContaining([
        expect.arrayContaining([
          [-1, -1, 1],
          [-2, -2, 2],
        ]),
        expect.arrayContaining([
          [1, -1, 1],
          [2, -2, 2],
        ]),
      ])
    );
  });

  it("works with very simple input", () => {
    const simpleInput =
      "--- scanner 0 ---\n-618,-824,-621\n-537,-823,-458\n\n--- scanner 1 ---\n686,422,578\n605,423,415";
    const result = solveChallenge(simpleInput);
    expect(result).toBe(2);
  });

  it("returns same result as example", () => {
    const result = solveChallenge(exampleInput);
    expect(result).toBe(79);
  });

  it("works with input.txt", () => {
    jest.setTimeout(500_000);

    const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8");
    const result = solveChallenge(input);
    console.log(result);
  });
});
