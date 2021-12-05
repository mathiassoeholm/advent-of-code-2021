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
        println!("start {:?}", start);
        println!("end {:?}", end);
        let is_horizontal = start.0 == end.0;
        let is_vertical = start.1 == end.1;

        if !is_vertical && !is_horizontal {
            continue;
        }

        for x in start.0..(end.0 + 1) {
            for y in start.1..(end.1 + 1) {
                let key = format!("{},{}", x, y);
                overlaps.insert(key.clone(), overlaps.get(&key).unwrap_or(&0) + 1);
            }
        }
    }

    let result: i32 = overlaps.values().filter(|&&v| v > 1).map(|_| 1).sum();

    println!("{}", result);
    // let lines = lines.map(||)
    // parse lines to
    // lines = Vec((start: (x:0,y:9), end: (x:5,y:9) ...)
    // overlapping = Vec<Vec<>>, fill with 0
    // for each line
    //   for each point it overlaps
    //     overlapping[x][y] += 1;
    //
    // result = overlapping.map(v => v.sum()).map(v => v.sum());
}
