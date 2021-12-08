fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input = input.split('\n').map(|entry| {
        let mut entry = entry.split(" | ");
        (entry.next().unwrap(), entry.next().unwrap())
    });

    let mut count = 0;
    for (signal_pattern, output) in input {
        let mut possibilities: Vec<Vec<char>> = (0..7)
            .map(|_| vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'])
            .collect();

        fn assign_possibility(signal: &str, index: usize, possibilities: &mut Vec<Vec<char>>) {
            possibilities[index] = possibilities[index]
                .clone()
                .into_iter()
                .filter(|&c| signal.contains(c))
                .collect();
        }

        fn prune_possibilities(possibilities: &mut Vec<Vec<char>>) {
            for i in 0..7 {
                possibilities[i] = possibilities[i]
                    .iter()
                    .filter(|&&c| {
                        !possibilities
                            .iter()
                            .enumerate()
                            .any(|(j, p)| j != i && p.len() == 1 && p[0] == c)
                    })
                    .map(|c| *c)
                    .collect()
            }
        }

        for signal in signal_pattern.split(' ') {
            match signal.len() {
                2 => {
                    assign_possibility(signal, 2, &mut possibilities);
                    assign_possibility(signal, 5, &mut possibilities);
                }
                3 => {
                    assign_possibility(signal, 0, &mut possibilities);
                    assign_possibility(signal, 2, &mut possibilities);
                    assign_possibility(signal, 5, &mut possibilities);
                }
                4 => {
                    assign_possibility(signal, 1, &mut possibilities);
                    assign_possibility(signal, 3, &mut possibilities);
                    assign_possibility(signal, 2, &mut possibilities);
                    assign_possibility(signal, 5, &mut possibilities);
                }

                // 2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }

            prune_possibilities(&mut &mut possibilities);
        }
        // [0].possibilities = union of [0].possibilities and new

        // After all possibilities assigned
        // Repeat until know all:
        // We are sure of some, remove those from possibilities of others

        for digit in output.split(' ') {
            match digit.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
        println!(
            "possibilities {:?}, signal_pattern {:?}",
            possibilities, signal_pattern
        );
    }
    println!("{}", count);
}
