use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut result = 0;
    let mut prev_three = Vec::with_capacity(3);
    for item in input.split("\n") {
        if item == "" {
            continue;
        }

        let item: i32 = item.parse().unwrap();
        if prev_three.len() == 3 {
            let prev_two_sum: i32 = prev_three.iter().skip(1).sum();
            if prev_two_sum + item > prev_three.iter().sum() {
                result += 1;
            };

            prev_three.remove(0);
        };

        prev_three.push(item);
    }
    println!("{}", result);
}
