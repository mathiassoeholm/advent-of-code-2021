fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut fish_school: Vec<_> = input
        .split(',')
        .map(|age| age.parse::<i32>().unwrap())
        .collect();

    for _ in 0..80 {
        let mut new_fish = Vec::new();
        for fish in fish_school.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }

        fish_school.append(&mut new_fish);
    }
    println!("{:?}", fish_school.len());
}
