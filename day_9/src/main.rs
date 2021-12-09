fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<_> = input.split("\n").collect();

    let row_size = input.first().unwrap().len();
    let col_size = input.len();

    let input: Vec<_> = input
        .iter()
        .flat_map(|line| line.split("").filter(|v| !v.is_empty()))
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let mut risk_level_sum = 0;

    for y in 0..col_size {
        for x in 0..row_size {
            let elem = input[y * row_size + x];

            let left = if x > 0 {
                input[y * row_size + x - 1]
            } else {
                i32::MAX
            };

            let top = if y > 0 {
                input[(y - 1) * row_size + x]
            } else {
                i32::MAX
            };

            let right = if x < row_size - 1 {
                input[y * row_size + x + 1]
            } else {
                i32::MAX
            };

            let bottom = if y < col_size - 1 {
                input[(y + 1) * row_size + x]
            } else {
                i32::MAX
            };

            if elem < left && elem < top && elem < right && elem < bottom {
                risk_level_sum += elem + 1;
            }
        }
    }
    println!("{}", risk_level_sum);
}
