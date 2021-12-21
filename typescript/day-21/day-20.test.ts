import { naiveSolution } from "./day-20";

describe("day 21", () => {
  it("works with example input", () => {
    const result = naiveSolution(4, 8);
    expect(result).toBe(739785);
  });

  it("works with my input", () => {
    const result = naiveSolution(2, 1);
    console.log(result);
  });
});
