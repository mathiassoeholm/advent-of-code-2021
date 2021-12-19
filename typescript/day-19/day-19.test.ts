import { getAllOrientations, getAllOrientationsForList } from "./day-19";

describe("day 19", () => {
  it("can get different orientations for a single position", () => {
    expect(getAllOrientations(-1, -1, 1)).toEqual(
      expect.arrayContaining([
        [-1, -1, 1],
        [1, -1, 1],
        [-1, -1, -1],
        [1, 1, -1],
        [1, 1, 1],
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
          [1, -1, 1],
        ]),
        expect.arrayContaining([
          [-2, -2, 2],
          [2, -2, 2],
        ]),
      ])
    );
  });
});
