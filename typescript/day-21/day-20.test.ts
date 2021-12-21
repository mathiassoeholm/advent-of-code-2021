import { naiveSolution, part2, rollDie } from "./day-20";

describe("day 21", () => {
  it("works with example input", () => {
    const result = naiveSolution(4, 8);
    expect(result).toBe(739785);
  });

  it("works with my input", () => {
    const result = naiveSolution(2, 1);
    console.log(result);
  });

  it("part 2 works", () => {
    part2(2, 1);
  });

  it("asfdf", () => {
    expect(rollDie(2, [10, 4, 12, 18, 2])).toEqual([2, 4, 14, 18, 3]);
    expect(rollDie(2, [10, 4, 12, 18, 1])).toEqual([2, 4, 12, 18, 2]);
    expect(rollDie(2, [10, 4, 12, 18, 4])).toEqual([10, 6, 12, 18, 5]);
    expect(rollDie(2, [10, 4, 12, 18, 5])).toEqual([10, 6, 12, 24, 0]);
  });
});
