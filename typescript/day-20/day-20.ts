export function parseInput(input: string) {
  const [decoder, image] = input.split("\n\n");
  return {
    decoder: decoder.split("").filter((c) => c !== "\n"),
    image,
  };
}
