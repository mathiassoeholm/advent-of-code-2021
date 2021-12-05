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
    for Line(start, end) in lines {
        println!("start {:?}", start);
        println!("end {:?}", end);
    }
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
