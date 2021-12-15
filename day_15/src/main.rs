fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let tile_size_x = input.split("\n").take(1).collect::<String>().len();
    let tile_size_y = input.split("\n").take(1).collect::<String>().len();
    let size_x = tile_size_x * 5;
    let size_y = tile_size_y * 5;
    let dest_x = size_x - 1;
    let dest_y = size_y - 1;

    let index_in_tile = |x: usize, y: usize| y * tile_size_x + x;
    let index = |x: usize, y: usize| y * size_x + x;

    let wieghts: Vec<_> = input
        .split('\n')
        .flat_map(|line| line.split("").filter(|v| !v.is_empty()))
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let get_weight = |x: usize, y: usize| {
        let extra_weight = ((x) / tile_size_x + (y) / tile_size_y) as i32;
        let weight = wieghts[index_in_tile(x % tile_size_x, y % tile_size_y)];
        return if weight + extra_weight > 9 {
            weight + extra_weight - 9
        } else {
            weight + extra_weight
        };
    };

    // for y in 0..size_y {
    //     for x in 0..size_x {
    //         print!("{}", get_weight(x, y));
    //     }
    //     print!("\n");
    // }

    let mut visited: Vec<_> = (0..(size_x * size_y)).map(|_| false).collect();
    let mut dist: Vec<_> = (0..(size_x * size_y)).map(|_| i32::MAX / 10).collect();
    dist[0] = 0;

    let mut current_x: usize = 0;
    let mut current_y: usize = 0;

    while !visited[index(dest_x, dest_y)] {
        for (offset_x, offset_y) in [(0i32, -1i32), (1, 0), (0, 1), (-1, 0)] {
            let neighbour_x = current_x as i32 + offset_x;
            let neighbour_y = current_y as i32 + offset_y;
            if neighbour_x < 0
                || neighbour_x == size_x as i32
                || neighbour_y < 0
                || neighbour_y == size_y as i32
            {
                continue;
            }
            let neighbour_index = index(neighbour_x as usize, neighbour_y as usize);
            if visited[neighbour_index] {
                continue;
            }

            let distance = dist[index(current_x, current_y)]
                + get_weight(neighbour_x as usize, neighbour_y as usize);
            if distance < dist[index(neighbour_x as usize, neighbour_y as usize)] {
                dist[index(neighbour_x as usize, neighbour_y as usize)] = distance;
            }
        }

        visited[index(current_x, current_y)] = true;
        let mut number_of_visited = 0;
        let mut smallest_distance = i32::MAX;
        for y in 0..size_y {
            for x in 0..size_x {
                if visited[index(x, y)] {
                    number_of_visited += 1;
                }

                if !visited[index(x, y)] && dist[index(x, y)] < smallest_distance {
                    smallest_distance = dist[index(x, y)];
                    current_x = x;
                    current_y = y;
                }
            }
        }

        // println!("current {} {}", current_x, current_y);

        // println!(
        //     "Progress: {}",
        //     (number_of_visited as f64) / (wieghts.len() as f64)
        // );
    }

    println!("Lowest risk path: {}", dist[index(dest_x, dest_y)]);
}
