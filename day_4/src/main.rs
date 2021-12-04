use std::cell::RefCell;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let first_line = input.split("\n").take(1).collect::<String>();
    let draw_numbers = first_line.split(",").map(|num| num.parse::<i32>().unwrap());

    let board_size = 5;

    let boards: Vec<Vec<_>> = input
        .split("\n\n")
        .skip(1)
        .map(|b| {
            b.split(&['\n', ' '][..])
                .filter(|&x| !x.is_empty())
                .map(|num| num.parse::<i32>().unwrap())
                .map(|num| (num, RefCell::new(false)))
                .collect()
        })
        .collect();

    let mut found_winner = false;
    for draw in draw_numbers {
        for board in &boards {
            for (num, marked) in board {
                if num == &draw {
                    marked.replace(true);
                }
            }

            // Check rows
            for y in 0..board_size {
                let mut is_marked = true;
                for x in 0..board_size {
                    is_marked = *board[y * board_size + x].1.borrow();
                    if !is_marked {
                        break;
                    }
                }

                if is_marked {
                    found_winner = true;
                    break;
                }
            }

            if !found_winner {
                // Check columns
                for x in 0..board_size {
                    let mut is_marked = true;
                    for y in 0..board_size {
                        is_marked = *board[y * board_size + x].1.borrow();
                        if !is_marked {
                            break;
                        }
                    }

                    if is_marked {
                        found_winner = true;
                        break;
                    }
                }
            }

            if found_winner {
                let score: i32 = board
                    .iter()
                    .filter(|(_, marked)| !*marked.borrow())
                    .map(|(num, _)| num)
                    .sum::<i32>()
                    * draw;
                println!("The score is {:?}", score);
                break;
            }
        }

        if found_winner {
            break;
        }
    }
}
