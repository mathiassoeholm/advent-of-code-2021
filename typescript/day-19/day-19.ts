import path from "path";
import fs from "fs";

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
        positions.push([x * i, z * j, y * k]);
        positions.push([y * i, x * j, z * k]);
        positions.push([y * i, z * j, x * k]);
        positions.push([z * i, x * j, y * k]);
        positions.push([z * i, y * j, x * k]);
      }
    }
  }

  return positions;
}

export function getAllOrientationsForList(
  positions: Position[]
): Array<Position[]> {
  const lists: Position[][] = [];
  positions.forEach((p) =>
    getAllOrientations(p[0], p[1], p[2]).forEach((p, i) => {
      if (!lists[i]) {
        lists.push([]);
      }

      lists[i].push(p);
    })
  );

  return lists;
}

export function solveChallenge(input: string) {
  const beacons = input.split("\n\n").map((text) =>
    text
      .split("\n")
      .slice(1)
      .map((line) => line.split(",").map((str) => parseInt(str)) as Position)
  );

  const absoluteBeacons = beacons[0];
  const relativeBeacons = beacons.slice(1);
  const scannerPositions = [[0, 0, 0]];

  for (let i = 0; i < 3; i++) {
    for (const beaconList of relativeBeacons) {
      let continueToNextScanner = false;
      const orientations = getAllOrientationsForList(beaconList);
      for (const orientedbeaconList of orientations) {
        for (const beaconRelative of orientedbeaconList) {
          for (const beaconAbsolute of absoluteBeacons) {
            const offset = [
              beaconAbsolute[0] - beaconRelative[0],
              beaconAbsolute[1] - beaconRelative[1],
              beaconAbsolute[2] - beaconRelative[2],
            ];

            const orientedBeaconListWithOffset = orientedbeaconList.map((p) => [
              p[0] + offset[0],
              p[1] + offset[1],
              p[2] + offset[2],
            ]);

            const beaconsInCommon = orientedBeaconListWithOffset.filter((b) =>
              absoluteBeacons.some(
                (a) => b[0] === a[0] && b[1] === a[1] && b[2] === a[2]
              )
            );
            if (beaconsInCommon.length >= 12) {
              if (
                !scannerPositions.some(
                  (p) =>
                    p[0] === offset[0] &&
                    p[1] === offset[1] &&
                    p[2] === offset[2]
                )
              ) {
                scannerPositions.push(offset);
              }
              orientedBeaconListWithOffset
                .filter((b) => !beaconsInCommon.includes(b))
                .forEach((p) => {
                  absoluteBeacons.push(p as Position);
                });
              continueToNextScanner = true;
            }
            if (continueToNextScanner) {
              break;
            }
          }
          if (continueToNextScanner) {
            break;
          }
        }
        if (continueToNextScanner) {
          break;
        }
      }
    }
  }

  let largestDistance = -Infinity;
  for (const [x1, y1, z1] of scannerPositions) {
    for (const [x2, y2, z2] of scannerPositions) {
      const distance =
        Math.abs(x1 - x2) + Math.abs(y1 - y2) + Math.abs(z1 - z2);
      if (distance > largestDistance) {
        largestDistance = distance;
      }
    }
  }
  console.log(largestDistance);

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
  return absoluteBeacons.length;
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

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8");
solveChallenge(input);
