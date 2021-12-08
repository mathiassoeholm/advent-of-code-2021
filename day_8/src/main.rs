fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input = input.split('\n').map(|entry| {
        let mut entry = entry.split(" | ");
        (entry.next().unwrap(), entry.next().unwrap())
    });

    let mut count = 0;
    for (signal_pattern, output) in input {
        let signals: Vec<_> = signal_pattern.split(' ').collect();
        let one = *signals.iter().find(|&s| s.len() == 2).unwrap();
        let four = *signals.iter().find(|&s| s.len() == 4).unwrap();
        let seven = *signals.iter().find(|&s| s.len() == 3).unwrap();
        let eight = *signals.iter().find(|&s| s.len() == 7).unwrap();

        let mut mapping = ['?'; 7];

        mapping[0] = seven.chars().find(|&c| !one.contains(c)).unwrap();
        mapping[2] = one
            .chars()
            .find(|&c| signals.iter().filter(|s| s.contains(c)).count() == 8)
            .unwrap();
        mapping[5] = one
            .chars()
            .find(|&c| signals.iter().filter(|s| s.contains(c)).count() == 9)
            .unwrap();
        mapping[1] = four
            .chars()
            .filter(|&c| !one.contains(c))
            .find(|&c| signals.iter().filter(|s| s.contains(c)).count() == 6)
            .unwrap();
        mapping[3] = four
            .chars()
            .find(|&c| !one.contains(c) && c != mapping[1])
            .unwrap();
        mapping[4] = eight
            .chars()
            .filter(|&c| !four.contains(c) || !seven.contains(c))
            .find(|&c| signals.iter().filter(|s| s.contains(c)).count() == 4)
            .unwrap();
        mapping[6] = eight
            .chars()
            .filter(|&c| !four.contains(c) || !seven.contains(c))
            .find(|&c| signals.iter().filter(|s| s.contains(c)).count() == 7)
            .unwrap();

        let mut number = String::new();
        for digit in output.split(' ') {
            if digit.len() == 2 {
                number += "1";
            } else if digit.len() == 5
                && digit.contains(mapping[0])
                && digit.contains(mapping[2])
                && digit.contains(mapping[3])
                && digit.contains(mapping[4])
                && digit.contains(mapping[6])
            {
                number += "2";
            } else if digit.len() == 5
                && digit.contains(mapping[0])
                && digit.contains(mapping[2])
                && digit.contains(mapping[3])
                && digit.contains(mapping[5])
                && digit.contains(mapping[6])
            {
                number += "3";
            } else if digit.len() == 4 {
                number += "4";
            } else if digit.len() == 5
                && digit.contains(mapping[0])
                && digit.contains(mapping[1])
                && digit.contains(mapping[3])
                && digit.contains(mapping[5])
                && digit.contains(mapping[6])
            {
                number += "5";
            } else if digit.len() == 6
                && digit.contains(mapping[0])
                && digit.contains(mapping[1])
                && digit.contains(mapping[3])
                && digit.contains(mapping[4])
                && digit.contains(mapping[5])
                && digit.contains(mapping[6])
            {
                number += "6";
            } else if digit.len() == 3 {
                number += "7";
            } else if digit.len() == 7 {
                number += "8";
            } else if digit.len() == 6
                && digit.contains(mapping[0])
                && digit.contains(mapping[1])
                && digit.contains(mapping[2])
                && digit.contains(mapping[3])
                && digit.contains(mapping[5])
                && digit.contains(mapping[6])
            {
                number += "9";
            } else {
                number += "0";
            }
        }

        count += number.parse::<i32>().unwrap();

        println!(
            "mapping {:?}, signal_pattern {:?}, number {:?}",
            mapping, signal_pattern, number
        );
    }
    println!("{}", count);
}
