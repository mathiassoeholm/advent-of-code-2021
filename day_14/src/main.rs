use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut input = input.split("\n\n");
    let template = input.next().unwrap();
    let rules: HashMap<_, _> = input
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut split = line.split(" -> ");
            (
                split.next().unwrap().to_owned(),
                split.next().unwrap().to_owned(),
            )
        })
        .collect();

    let mut element_count = HashMap::new();

    let mut pair_map = HashMap::new();
    for i in 0..(template.len()) {
        let char = &template[i..(i + 1)];
        element_count.insert(
            char.to_owned(),
            element_count.get(char).unwrap_or(&0) + 1 as u64,
        );
        if i > 0 {
            let key = format!(
                "{}{}",
                (&template[(i - 1)..(i)]).to_owned(),
                (&template[i..(i + 1)]).to_owned()
            );
            let value = pair_map.get(&key).unwrap_or(&0);
            pair_map.insert(key, value + 1 as u64);
        }
    }

    for _ in 0..40 {
        let mut new_pair_map = HashMap::new();
        for (pair, count) in pair_map.clone() {
            if let Some(letter) = rules.get(&pair) {
                let new_pair_1 = format!("{}{}", &pair[0..1], letter);
                let new_pair_2 = format!("{}{}", letter, &pair[1..2]);

                element_count.insert(
                    letter.clone(),
                    element_count.get(letter).unwrap_or(&0) + count,
                );

                let value_1 = new_pair_map.get(&new_pair_1).unwrap_or(&0) + count;
                let value_2 = new_pair_map.get(&new_pair_2).unwrap_or(&0) + count;
                new_pair_map.insert(new_pair_1, value_1);
                new_pair_map.insert(new_pair_2, value_2);
            } else {
                new_pair_map.insert(pair, count);
            }
        }
        pair_map = new_pair_map;
    }
    let most_common_count = element_count.values().max().unwrap();
    let least_common_count = element_count.values().min().unwrap();
    let result = most_common_count - least_common_count;

    println!("Hello, world! {}", result);
}
