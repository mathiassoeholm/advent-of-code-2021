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
  return false;
}
