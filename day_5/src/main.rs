use std::collections::HashMap;

#[derive(Debug)]
struct Point(i32, i32);
struct Line(Point, Point);

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines = input.split('\n');

    let lines = lines.map(|line| {
        let mut split = line.split(" -> ");
        let mut start = split
            .next()
            .unwrap()
            .split(',')
            .map(|str| str.parse::<i32>().unwrap());
        let mut end = split
            .next()
            .unwrap()
            .split(',')
            .map(|str| str.parse::<i32>().unwrap());
        Line(
            Point(start.next().unwrap(), start.next().unwrap()),
            Point(end.next().unwrap(), end.next().unwrap()),
        )
    });

    let mut overlaps = HashMap::new();

    for Line(start, end) in lines {
        let is_horizontal = start.0 == end.0;
        let is_vertical = start.1 == end.1;

        if is_vertical || is_horizontal {
            for x in std::cmp::min(start.0, end.0)..(std::cmp::max(start.0, end.0) + 1) {
                for y in std::cmp::min(start.1, end.1)..(std::cmp::max(start.1, end.1) + 1) {
                    let key = format!("{},{}", x, y);
                    overlaps.insert(key.clone(), overlaps.get(&key).unwrap_or(&0) + 1);
                }
            }
        } else {
            let x = std::cmp::min(start.0, end.0);
            let y = if start.0 < end.0 { start.1 } else { end.1 };
            let y_increment = if start.0 < end.0 {
                if end.1 > start.1 {
                    1
                } else {
                    -1
                }
            } else {
                if start.1 > end.1 {
                    1
                } else {
                    -1
                }
            };
            for i in 0..((start.0 - end.0).abs() + 1) {
                let key = format!("{},{}", x + i, y + i * y_increment);
                overlaps.insert(key.clone(), overlaps.get(&key).unwrap_or(&0) + 1);
            }
        }
    }

    let result: i32 = overlaps.values().filter(|&&v| v > 1).map(|_| 1).sum();

    println!("{}", result);
}
