use aoc::load_input;

fn main() {
    let lines = load_input();

    println!("Result: {}", calculate(lines));
}

fn calculate(input: Vec<String>) -> u64 {
    let mut input_iter = input.iter();

    let initial = input_iter.next().unwrap();
    println!("Initial: {}", initial);

    let mut res: Vec<usize> = initial
        .chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect();

    println!("Res: {:?}", res);

    for l in input_iter {
        let c: Vec<char> = l.chars().collect();
        for j in 0..c.len() {
            match c[j] {
                '0' => (),
                '1' => res[j] += 1,
                _ => panic!("Hmm..."),
            }
        }
    }

    let mut result: u64 = 0;
    let mut mask = 0;

    for i in 0..res.len() {
        result = result << 1;
        if res[i] > input.len() / 2 {
            result += 1;
        }
        mask = mask << 1;
        mask += 1;
    }

    result * (!result & mask)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prompt_input() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(calculate(input), 198);
    }
}
