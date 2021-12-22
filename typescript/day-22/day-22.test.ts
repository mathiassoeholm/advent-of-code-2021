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
    const volume = "on x=-46..7,y=-6..46,z=-50..-1";
    expect(doesOverlap([-50, 0, 0], volume)).toBe(false);
    expect(doesOverlap([-46, 0, 0], volume)).toBe(true);
    expect(doesOverlap([-5, 7, 0], volume)).toBe(true);
    expect(doesOverlap([-5, 7, 2], volume)).toBe(false);
    expect(doesOverlap([-5, 47, 0], volume)).toBe(false);
  });
});
