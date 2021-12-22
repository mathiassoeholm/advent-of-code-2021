import fs from "fs";
import path from "path";
import { isInVolume, parseVolume, solvePart1, solvePart2 } from "./day-22";

describe("day 22", () => {
  it("should parse volume", () => {
    expect(parseVolume("on x=-46..7,y=-6..46,z=-50..-1")).toEqual({
      xMin: -46,
      xMax: 7,
      yMin: -6,
      yMax: 46,
      zMin: -50,
      zMax: -1,
      on: true,
    });
  });

  it("should check if overlaps", () => {
    const volume = parseVolume("on x=-46..7,y=-6..46,z=-50..-1");
    expect(isInVolume([-50, 0, -1], volume)).toBe(false);
    expect(isInVolume([-46, 0, -1], volume)).toBe(true);
    expect(isInVolume([-5, 7, -1], volume)).toBe(true);
    expect(isInVolume([-5, 7, 0], volume)).toBe(false);
    expect(isInVolume([-5, 47, -1], volume)).toBe(false);
  });

  it("works on example input", () => {
    const input = fs.readFileSync(
      path.join(__dirname, "example-input.txt"),
      "utf-8"
    );

    const result = solvePart1(input);
    expect(result).toBe(590784);
  });

  it("works on input", () => {
    const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8");

    const result = solvePart1(input);
    console.log("Result", result);
  });

  it("can solve part 2 on example input", () => {
    const input = fs.readFileSync(
      path.join(__dirname, "example-input-2.txt"),
      "utf-8"
    );

    const result = solvePart2(input);
    expect(result).toBe(2758514936282235);
  });
});
