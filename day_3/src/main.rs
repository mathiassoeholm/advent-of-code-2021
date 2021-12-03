use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let all_bits: Vec<_> = input
        .split('\n')
        .filter(|&line| line != "")
        .map(|line| line.split("").filter(|&x| !x.is_empty()))
        .collect();
    let num_bits = all_bits.len();

    let mut sums = Vec::new();

    for bits in all_bits {
        for (index, bit) in bits.enumerate() {
            if sums.len() <= index {
                sums.push(bit.parse::<usize>().unwrap());
            }

            if bit == "1" {
                sums[index] = sums[index] + 1;
            }
        }
    }

    let gamma = sums
        .iter()
        .map(|&s| if s > num_bits / 2 { "1" } else { "0" })
        .fold(String::new(), |a, b| a + b);
    let gamma = isize::from_str_radix(&gamma, 2).unwrap();

    let epsilon = sums
        .iter()
        .map(|&s| if s > num_bits / 2 { "0" } else { "1" })
        .fold(String::new(), |a, b| a + b);
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{:?}", gamma);
    println!("{:?}", epsilon);
    println!("{:?}", gamma * epsilon);
}
