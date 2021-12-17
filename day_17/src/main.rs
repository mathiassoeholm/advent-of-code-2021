use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let captures = Regex::new(r"x=(\d+)\.\.(\d+), y=(-?\d+)\.\.(-?\d+)")
        .unwrap()
        .captures(&input)
        .unwrap();
    let min_x = &captures[1].parse::<i32>().unwrap();
    let max_x = &captures[2].parse::<i32>().unwrap();
    let min_y = &captures[3].parse::<i32>().unwrap();
    let max_y = &captures[4].parse::<i32>().unwrap();

    println!("Hello, world! {} {} {} {}", min_x, max_x, min_y, max_y);
}
