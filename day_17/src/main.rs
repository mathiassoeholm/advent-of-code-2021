use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let captures = Regex::new(r"x=(\d+)\.\.(\d+), y=(-?\d+)\.\.(-?\d+)")
        .unwrap()
        .captures(&input)
        .unwrap();
    let min_x = *&captures[1].parse::<i32>().unwrap();
    let max_x = *&captures[2].parse::<i32>().unwrap();
    let min_y = *&captures[3].parse::<i32>().unwrap();
    let max_y = *&captures[4].parse::<i32>().unwrap();

    let mut y_attempts: Vec<_> = (0..10000).collect();
    y_attempts.reverse();

    for initial_y_velocity in y_attempts {
        let mut is_in_target_area = false;

        for initial_x_velocity in 1..max_x {
            // println!(
            //     "Trying with new x {} y {}",
            //     initial_x_velocity, initial_y_velocity
            // );

            let mut did_overshoot = false;
            let mut did_undershoot = false;

            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut velocity_x = initial_x_velocity;
            let mut velocity_y = initial_y_velocity;

            loop {
                x_pos += velocity_x;
                y_pos += velocity_y;
                velocity_x = std::cmp::max(0, velocity_x - 1);
                velocity_y -= 1;

                // println!("x pos {} veloc {}", x_pos, velocity_x);

                did_overshoot = x_pos > max_x || y_pos < min_y;
                did_undershoot = velocity_x == 0 && x_pos < min_x;
                is_in_target_area =
                    x_pos >= min_x && x_pos <= max_x && y_pos >= min_y && y_pos <= max_y;

                if is_in_target_area || did_overshoot || did_undershoot {
                    break;
                }
            }

            if is_in_target_area {
                break;
            }
        }

        if is_in_target_area {
            println!("y is {}", initial_y_velocity);
            break;
        }
    }

    println!("Hello, world! {} {} {} {}", min_x, max_x, min_y, max_y);
}
