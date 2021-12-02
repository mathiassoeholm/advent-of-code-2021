use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut result = 0;
    let mut prev = None;
    for item in input.split("\n") {
        if item == "" {
            continue;
        }

        let item: i32 = item.parse().unwrap();
        if let Some(prev_item) = prev {
            if item > prev_item {
                result += 1;
            };
        };

        prev = Some(item);
    }
    println!("{}", result);
}
