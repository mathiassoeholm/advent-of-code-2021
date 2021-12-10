use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input = input.split('\n');

    let bracket_pair = HashMap::<_, _>::from_iter([('[', ']'), ('(', ')'), ('<', '>'), ('{', '}')]);

    let reverse_brack_pair: HashMap<_, _> = bracket_pair
        .keys()
        .map(|&key| (*bracket_pair.get(&key).unwrap(), key))
        .collect();

    let score_map =
        HashMap::<_, u64>::from_iter(IntoIter::new([(']', 2), (')', 1), ('>', 4), ('}', 3)]));

    let mut scores = Vec::new();

    for line in input {
        let mut open_brackets = Vec::new();
        let mut corrupt = false;
        for bracket in line.chars() {
            match bracket {
                '[' | '(' | '{' | '<' => open_brackets.push(bracket),
                x if reverse_brack_pair.keys().any(|&k| k == x) => {
                    if open_brackets.last() != reverse_brack_pair.get(&x) {
                        corrupt = true;
                        break;
                    }

                    open_brackets.pop();
                }
                _ => {}
            }
        }

        if !corrupt {
            open_brackets.reverse();
            let score = open_brackets
                .iter()
                .map(|c| bracket_pair.get(c).unwrap())
                .fold(0, |acc, x| acc * 5 + score_map.get(x).unwrap());
            scores.push(score);
        }
    }

    scores.sort();
    let middle_score = scores[scores.len() / 2];
    println!("{}", middle_score)
}
