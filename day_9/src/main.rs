fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<_> = input.split("\n").collect();

    let row_size = input.first().unwrap().len();
    let col_size = input.len();

    let input: Vec<_> = input
        .iter()
        .flat_map(|line| line.split("").filter(|v| !v.is_empty()))
        .map(|v| v.parse::<u8>().unwrap())
        .collect();

    let mut bassin_sizes = Vec::new();

    for y in 0..col_size {
        for x in 0..row_size {
            let elem = input[y * row_size + x];

            let left = if x > 0 {
                input[y * row_size + x - 1]
            } else {
                10
            };

            let top = if y > 0 {
                input[(y - 1) * row_size + x]
            } else {
                10
            };

            let right = if x < row_size - 1 {
                input[y * row_size + x + 1]
            } else {
                10
            };

            let bottom = if y < col_size - 1 {
                input[(y + 1) * row_size + x]
            } else {
                10
            };

            if elem < left && elem < top && elem < right && elem < bottom {
                fn visit_neighbours(
                    row_size: isize,
                    col_size: isize,
                    input: &Vec<u8>,
                    coordinate: (isize, isize),
                    nodes: &mut Vec<(isize, isize)>,
                ) {
                    let index = coordinate.1 * row_size + coordinate.0;

                    if coordinate.0 < 0
                        || coordinate.1 < 0
                        || coordinate.0 >= row_size as isize
                        || coordinate.1 >= col_size as isize
                        || input[index as usize] >= 9
                        || nodes.contains(&coordinate)
                    {
                        return;
                    }

                    nodes.push(coordinate);
                    for neighbor in [
                        (coordinate.0 - 1, coordinate.1),
                        (coordinate.0, coordinate.1 - 1),
                        (coordinate.0 + 1, coordinate.1),
                        (coordinate.0, coordinate.1 + 1),
                    ] {
                        visit_neighbours(row_size, col_size, input, neighbor, nodes);
                    }
                }

                let mut nodes = Vec::new();
                visit_neighbours(
                    row_size as isize,
                    col_size as isize,
                    &input,
                    (x as isize, y as isize),
                    &mut nodes,
                );

                bassin_sizes.push(nodes.len());
            }
        }
    }

    bassin_sizes.sort_by(|a, b| b.cmp(a));
    let largest: usize = bassin_sizes.iter().take(3).product();
    println!("{:?}", largest);
}
