use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    let mut words: Vec<Vec<_>> = input
        .split('\n')
        .filter(|&line| line != "")
        .map(|line| line.split("").filter(|&x| !x.is_empty()).collect())
        .collect();

    fn get_reading(most_common: bool, words: Vec<Vec<&str>>) {
        let word_len = words[0].len();

        for col in 0..word_len {
            let sum: f64 = words
                .iter()
                .map(|word| word[col])
                .map(|bit| bit.parse::<f64>().unwrap())
                .sum();

            let bit_to_match = if most_common {
                if sum >= (words.len() as f64) / 2.0 {
                    1
                } else {
                    0
                }
            } else {
                if sum <= (words.len() as f64) / 2.0 {
                    0
                } else {
                    1
                }
            };

            println!("bit to match {:?}", bit_to_match)
        }
    }

    get_reading(true, words);
}
