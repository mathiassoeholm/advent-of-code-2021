fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let crabs: Vec<_> = input
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let max_pos = *crabs.iter().max().unwrap();
    let min_pos = *crabs.iter().min().unwrap();

    let mut lowest_possible = std::i32::MAX;

    for i in min_pos..max_pos {
        let mut moves = 0;
        for crab in &crabs {
            let n = (i - crab).abs();
            moves += n * (n + 1) / 2;
        }

        if moves < lowest_possible {
            lowest_possible = moves;
        }
    }

    println!("{}", lowest_possible);
}
