export type Position = [x: number, y: number, z: number];

export function getAllOrientations(
  x: number,
  y: number,
  z: number
): Position[] {
  const positions: Array<Position> = [];

  for (const i of [-1, 1]) {
    for (const j of [-1, 1]) {
      for (const k of [-1, 1]) {
        positions.push([x * i, y * j, z * k]);
        positions.push([z * i, x * j, y * k]);
        positions.push([y * i, z * j, x * k]);
      }
    }
  }

  return positions;
}

export function getAllOrientationsForList(
  positions: Position[]
): Array<Position[]> {
  return positions.map((p) => getAllOrientations(p[0], p[1], p[2]));
}

export function solveChallenge(input: string) {
  const beacons = input.split("\n\n").map((text) =>
    text
      .split("\n")
      .slice(1)
      .map((line) => line.split(",").map((str) => parseInt(str)))
  );

  const absoluteBeacons = new Set(beacons[0]);
  const relativeBeacons = beacons.slice(1);

  console.log(beacons);
  // const absoluteBeacons = new Set with all beacons from scanner 0
  // const relativeBeacons = [scanner][beacon]
  //
  // for each beaconList in relativeBeacons
  //   const orientations = getAllOrientations(beaconList)
  //   for each beaconListOriented in orientations
  //     for each beaconRel in beaconListOriented
  //       for each beaconAbs in absoluteBeacons
  //         const offset - beaconAbs - beaconRel
  //         const relBeaconsOffsetted = beaconListOriented.map(use offset)
  //         if relBeaconsOffsetted.filter(b => b is in absoluteBeacons).count >= 12
  //            we have a winner!
  //            Add relBeaconsOffsetted to absoluteBeacons
  //
  // result is = size of set absoluteBeacons
}

// let restScanners = [all others]
// for each scanner a in scannersRelativeTo0
// for each scanner b in restScanners
//   for all orientations of b
//     for all beacons in b
//       for all beacons in a
//         offset all beacons in b by beacon_a-beacon_b
//         count the overlaps
//         if overlap >= 12:
//            we have a pair
//            calculate scanner b pos relative to scanner a
//            save in scannersRelativeTo0
//            remove from restScanners
//
