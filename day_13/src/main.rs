use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut split = input.split("\n\n");
    let holes: HashSet<(_, _)> = split
        .next()
        .unwrap()
        .split('\n')
        .map(|hole| {
            let mut split = hole.split(',').map(|v| v.parse::<i32>().unwrap());
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();
    let folds = split.next().unwrap().split('\n').map(|line| {
        let mut split = line.split(' ').last().unwrap().split('=');
        (
            split.next().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        )
    });

    for (axis, val) in folds {
        println!("{} {}", axis, val);
    }

    println!("Hello, world! {:?}", holes);
}
