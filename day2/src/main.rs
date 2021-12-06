use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let input_file = std::env::args().nth(1).expect("no input given");
    let input_path = Path::new(&input_file);

    let mut dc = DiveCalculator::new();

    for l in read_lines(input_path)?
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
    {
        dc.process_move(l)
    }

    // Set inc to 1 for first part.
    println!("Dive Calculation: {}", dc.position());

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct DiveCalculator {
    x: i64,
    z: i64,
    aim: i64,
}

impl DiveCalculator {
    fn new() -> Self {
        DiveCalculator { x: 0, z: 0, aim: 0 }
    }
    fn process_move(&mut self, line: String) {
        let l: Vec<&str> = line.split(" ").collect();

        match (l[0], l[1]) {
            ("forward", n) => {
                let x = n.parse::<i64>().unwrap();
                self.x += x;
                self.z += self.aim * x;
            }
            ("down", n) => self.aim += n.parse::<i64>().unwrap(),
            ("up", n) => self.aim -= n.parse::<i64>().unwrap(),
            (_, _) => panic!("Hmm..."),
        };
    }

    fn position(&self) -> i64 {
        self.x * self.z
    }
}

#[cfg(test)]
mod test {
    use crate::DiveCalculator;

    #[test]
    fn test_input() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let mut dc = DiveCalculator::new();
        for line in input {
            dc.process_move(line.to_owned());
        }
        // assert_eq!(dc.position(), 150);
        assert_eq!(dc.position(), 900);
    }
}
