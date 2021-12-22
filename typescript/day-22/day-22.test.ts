import { isInVolume, parseVolume } from "./day-22";

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
});
