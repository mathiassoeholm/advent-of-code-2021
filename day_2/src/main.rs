use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let instructions = input.split('\n').filter(|&line| line != "").map(|line| {
        let split: Vec<_> = line.split(' ').collect();
        (split[0], split[1].parse::<i32>().unwrap())
    });

    let mut aim = 0;
    let mut depth = 0;
    let mut hoz = 0;

    for (direction, distance) in instructions {
        match direction {
            "forward" => {
                hoz += distance;
                depth += aim * distance;
            }
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => panic!("Unknown direction {}", direction),
        }
    }

    println!("{:?}", depth * hoz);
}
