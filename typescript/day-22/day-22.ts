// For each line
// Run through overlapping cubes
// toggle them

interface Volume {
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
  volume: Volume
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
