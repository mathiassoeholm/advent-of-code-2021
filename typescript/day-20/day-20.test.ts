import path from "path";
import fs from "fs";
import { parseInput } from "./day-20";

describe("day 20", () => {
  it("can parse input", () => {
    const parsed = parseInput("..#.#\n###.\n\n#.\n.#");
    expect(parsed).toEqual({
      decoder: [".", ".", "#", ".", "#", "#", "#", "#", "."],
      image: "#.\n.#",
    });
  });

  // it("works with input.txt", () => {
  //   const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8");
  //   const result = solveChallenge(input);
  //   console.log(result);
  // });
});
