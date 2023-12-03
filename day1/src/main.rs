fn main() {
    let input = include_str!("../input");
    let part1 = part1(input);
    let part2 = part2(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // This can be done more efficiently with a first and second variable, see parse_line (part2)
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

            digits[0].to_digit(10).unwrap() * 10 + digits.last().unwrap().to_digit(10).unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> u32 {
    let (mut first, mut second) = (None, None);
    for line in (0..line.len()).map(|i| &line[..i + 1]) {
        for s in [
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three",
            "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        {
            if line.ends_with(s) {
                let digit = match *s {
                    "0" | "zero" => 0,
                    "1" | "one" => 1,
                    "2" | "two" => 2,
                    "3" | "three" => 3,
                    "4" | "four" => 4,
                    "5" | "five" => 5,
                    "6" | "six" => 6,
                    "7" | "seven" => 7,
                    "8" | "eight" => 8,
                    "9" | "nine" => 9,
                    _ => unreachable!(),
                };

                if first.is_none() {
                    first = Some(digit);
                } else {
                    second = Some(digit);
                }
            }
        }
    }

    first.unwrap() * 10 + second.unwrap_or(first.unwrap())
}
