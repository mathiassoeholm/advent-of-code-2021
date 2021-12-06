fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let fish_school = input.split(',').map(|age| age.parse::<u64>().unwrap());

    let mut fish_map = vec![0; 9];

    for fish in fish_school {
        fish_map[fish as usize] += 1;
    }

    for day in 0..256 {
        let mut new_fish = vec![0; 9];

        new_fish[6] = fish_map[0];
        new_fish[8] = fish_map[0];
        for i in 1..9 {
            new_fish[i - 1] = new_fish[i - 1] + fish_map[i];
        }

        fish_map = new_fish;
    }

    println!("fish sum {}", fish_map.iter().sum::<u64>())
}
