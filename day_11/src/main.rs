fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut oct: Vec<_> = input
        .split('\n')
        .flat_map(|line| line.split("").filter(|v| !v.is_empty()))
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut flashed: Vec<_> = oct.iter().map(|_| false).collect();
    let mut num_flashes = 0;

    for step in 0..10000 {
        for y in 0..10 {
            for x in 0..10 {
                oct[y * 10 + x] += 1;
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                if oct[y * 10 + x] > 9 && !flashed[y * 10 + x] {
                    flash(x, y, &mut oct, &mut flashed, &mut num_flashes)
                }
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                if flashed[y * 10 + x] {
                    oct[y * 10 + x] = 0;
                    flashed[y * 10 + x] = false
                }
            }
        }

        if oct.iter().all(|&v| v == 0) {
            println!("step {}", step);
            break;
        }
    }

    fn flash(
        x: usize,
        y: usize,
        oct: &mut Vec<u8>,
        flashed: &mut Vec<bool>,
        num_flashes: &mut i32,
    ) {
        // println!("Already flashed {}", flashed[y * 10 + x]);
        flashed[y * 10 + x] = true;
        *num_flashes += 1;
        for (offset_x, offset_y) in [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ] {
            let neighbour_x = x as i32 + offset_x;
            let neighbour_y = y as i32 + offset_y;
            if neighbour_x < 0 || neighbour_x == 10 || neighbour_y < 0 || neighbour_y == 10 {
                continue;
            }

            oct[(neighbour_y * 10 + neighbour_x) as usize] += 1;
            if oct[(neighbour_y * 10 + neighbour_x) as usize] > 9
                && !flashed[(neighbour_y * 10 + neighbour_x) as usize]
            {
                flash(
                    neighbour_x as usize,
                    neighbour_y as usize,
                    oct,
                    flashed,
                    num_flashes,
                );
            }
        }
    }

    println!("{}", num_flashes);
}
