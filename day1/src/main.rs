use aoc::load_input;

fn main() -> std::io::Result<()> {
    let lines = load_input()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // Set inc to 1 for first part.
    println!("Depth: {}", inc_cnt(lines, 3));

    Ok(())
}

fn inc_cnt(input: Vec<i64>, increment: usize) -> usize {
    let mut count = 0;

    for i in 0..input.len() - increment {
        if input[i] < input[i + increment] {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_input() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(inc_cnt(input.clone(), 1), 7);
        assert_eq!(inc_cnt(input.clone(), 3), 5);
    }
}
