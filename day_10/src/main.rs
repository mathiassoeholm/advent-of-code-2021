use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input = input.split('\n');

    let mut errors = Vec::new();

    let bracket_pair = HashMap::<_, _>::from_iter(IntoIter::new([
        (']', '['),
        (')', '('),
        ('>', '<'),
        ('}', '{'),
    ]));

    let scores = HashMap::<_, _>::from_iter(IntoIter::new([
        (']', 57),
        (')', 3),
        ('>', 25137),
        ('}', 1197),
    ]));

    for line in input {
        let mut open_brackets = Vec::new();
        for bracket in line.chars() {
            match bracket {
                '[' | '(' | '{' | '<' => open_brackets.push(bracket),
                x if bracket_pair.keys().any(|&k| k == x) => {
                    if open_brackets.last() != bracket_pair.get(&x) {
                        errors.push(x);
                        break;
                    }

                    open_brackets.pop();
                }
                _ => {}
            }
        }
    }

    println!(
        "{:?}",
        errors.iter().map(|e| scores.get(e).unwrap()).sum::<i32>()
    );
}
