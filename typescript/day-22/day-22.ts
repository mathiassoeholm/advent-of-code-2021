// For each line
// Run through overlapping cubes
// toggle them

export interface Volume {
  xMin: number;
  xMax: number;
  yMin: number;
  yMax: number;
  zMin: number;
  zMax: number;
  on: boolean;
}

export function parseVolume(str: string): Volume {
  const match = str.match(
    /^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)/
  )!;
  return {
    xMin: parseInt(match[2]),
    xMax: parseInt(match[3]),
    yMin: parseInt(match[4]),
    yMax: parseInt(match[5]),
    zMin: parseInt(match[6]),
    zMax: parseInt(match[7]),
    on: match[1] === "on",
  };
}

export function isInVolume(
  [x, y, z]: [number, number, number],
  volume: Omit<Volume, "on">
) {
  return (
    x >= volume.xMin &&
    x <= volume.xMax &&
    y >= volume.yMin &&
    y <= volume.yMax &&
    z >= volume.zMin &&
    z <= volume.zMax
  );
}

export function solvePart1(input: string) {
  const volumes = input.split("\n").map(parseVolume);
  const cubes = new Map<string, boolean>();

  for (const volume of volumes) {
    for (
      let x = Math.max(volume.xMin, -50);
      x <= Math.min(volume.xMax, 50);
      x++
    ) {
      for (
        let y = Math.max(volume.yMin, -50);
        y <= Math.min(volume.yMax, 50);
        y++
      ) {
        for (
          let z = Math.max(volume.zMin, -50);
          z <= Math.min(volume.zMax, 50);
          z++
        ) {
          if (isInVolume([x, y, z], volume)) {
            cubes.set([x, y, z].join(","), volume.on);
          }
        }
      }
    }
  }

  return [...cubes.values()].filter(Boolean).length;
}

/**
 * Returns true if volumeA is completely contained by volumeB.
 */
export function completeleyCovers(
  volumeA: Omit<Volume, "on">,
  volumeB: Omit<Volume, "on">
) {
  return (
    isInVolume([volumeA.xMin, volumeA.yMin, volumeA.zMin], volumeB) &&
    isInVolume([volumeA.xMax, volumeA.yMax, volumeA.zMax], volumeB)
  );
}

export function overlaps(
  volumeA: Omit<Volume, "on">,
  volumeB: Omit<Volume, "on">
) {
  return (
    volumeA.xMax > volumeB.xMin &&
    volumeA.xMin < volumeB.xMax &&
    volumeA.yMax > volumeB.yMin &&
    volumeA.yMin < volumeB.yMax &&
    volumeA.zMax > volumeB.zMin &&
    volumeA.zMin < volumeB.zMax
  );
}

export function solvePart2(input: string) {
  const volumesReverse = input.split("\n").map(parseVolume).reverse();

  function countOn(volumeToTest: Omit<Volume, "on">): number {
    for (const volume of volumesReverse) {
      if (completeleyCovers(volumeToTest, volume)) {
        // Return size of volume if volume.on otherwise 0
        return volume.on
          ? (volumeToTest.xMax - volumeToTest.xMin) *
              (volumeToTest.yMax - volumeToTest.yMin) *
              (volumeToTest.zMax - volumeToTest.zMin)
          : 0;
      } else if (overlaps(volumeToTest, volume)) {
        break;
      }
    }

    // Call recursively
    const size = {
      x: (volumeToTest.xMax - volumeToTest.xMin) / 2,
      y: (volumeToTest.yMax - volumeToTest.yMin) / 2,
      z: (volumeToTest.zMax - volumeToTest.zMin) / 2,
    };
    const smallerVolumes = [];
    for (let x = 0; x <= 1; x++) {
      for (let y = 0; y <= 1; y++) {
        for (let z = 0; z <= 1; z++) {
          smallerVolumes.push({
            xMin: volumeToTest.xMin + size.x * x,
            xMax: volumeToTest.xMin + size.x * x + size.x,
            yMin: volumeToTest.yMin + size.y * y,
            yMax: volumeToTest.yMin + size.y * y + size.y,
            zMin: volumeToTest.zMin + size.z * z,
            zMax: volumeToTest.zMin + size.z * z + size.z,
          });
        }
      }
    }

    // console.log("hey");

    return smallerVolumes.map(countOn).reduce((a, b) => a + b, 0);
  }

  // for entire
  // search bottom up for area that completely covers volume, fail if something partially covers it
  // if fail, call recursively on area split in eight smaller cubes
  // if

  // For each volume
  // if volume is on
  //   Add all cubes to on-list that are not overlapped by next volumes
  //
  return countOn({
    xMin: Math.min(...volumesReverse.map(({ xMin }) => xMin)),
    xMax: Math.max(...volumesReverse.map(({ xMax }) => xMax)),
    yMin: Math.min(...volumesReverse.map(({ yMin }) => yMin)),
    yMax: Math.max(...volumesReverse.map(({ yMax }) => yMax)),
    zMin: Math.min(...volumesReverse.map(({ zMin }) => zMin)),
    zMax: Math.max(...volumesReverse.map(({ zMax }) => zMax)),
  });
}
