use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut split = input.split("\n\n");
    let mut holes: HashSet<(_, _)> = split
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
        for (x, y) in holes.clone() {
            if axis == "x" && x > val {
                holes.remove(&(x, y));
                holes.insert((val - (x - val), y));
            } else if axis == "y" && y > val {
                holes.remove(&(x, y));
                holes.insert((x, val - (y - val)));
            }
        }
    }

    let max_x = *holes.iter().map(|(x, y)| x).max().unwrap();
    let max_y = *holes.iter().map(|(x, y)| y).max().unwrap();
    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if holes.contains(&(x, y)) {
                print!("💩");
            } else {
                print!("🧻");
            }
        }
        print!("\n");
    }

    println!("Hello, world! {:?}", holes.len());
}
