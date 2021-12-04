use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    // let input =
    //     "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    let words: Vec<Vec<_>> = input
        .split('\n')
        .filter(|&line| line != "")
        .map(|line| line.split("").filter(|&x| !x.is_empty()).collect())
        .collect();

    fn get_reading(most_common: bool, mut words: Vec<Vec<&str>>) -> String {
        let word_len = words[0].len();

        for col in 0..word_len {
            let sum: f64 = words
                .iter()
                .map(|word| word[col])
                .map(|bit| bit.parse::<f64>().unwrap())
                .sum();

            let bit_to_match = if most_common {
                if sum >= words.len() as f64 / 2.0 {
                    "1"
                } else {
                    "0"
                }
            } else {
                if sum >= words.len() as f64 / 2.0 {
                    "0"
                } else {
                    "1"
                }
            };

            words = words
                .into_iter()
                .filter(|word| word[col] == bit_to_match)
                .collect();

            if words.len() == 1 {
                return words[0]
                    .iter()
                    .map(|&str| String::from(str))
                    .collect::<String>();
            }
        }

        return String::from("error");
    }

    let oxygen = get_reading(true, words.clone());
    let oxygen = isize::from_str_radix(&oxygen, 2).unwrap();
    println!("{:?}", oxygen);

    let co2 = get_reading(false, words.clone());
    let co2 = isize::from_str_radix(&co2, 2).unwrap();
    println!("{:?}", co2);

    let life_support = oxygen * co2;
    println!("{:?}", life_support);
}
