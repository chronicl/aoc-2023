#![feature(iter_array_chunks)]

fn main() {
    let input = include_str!("../input");
    let part1 = part1(input);
    let part2 = part2(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| filter_line(line)).sum()
}

fn filter_line(line: &str) -> u32 {
    let mut tokens = line.split(' ').skip(1);
    let id = tokens.next().unwrap();
    let is_valid = tokens.array_chunks::<2>().all(|pair| {
        let (count, color) = (pair[0].parse::<u32>().unwrap(), pair[1]);
        count
            <= match color.chars().next().unwrap() {
                'r' => 12,
                'g' => 13,
                'b' => 14,
                _ => unreachable!(),
            }
    });
    if is_valid {
        id[..id.len() - 1].parse::<u32>().unwrap()
    } else {
        0
    }
}

fn part2(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let mut rgb = [0u32; 3];
    for pair in line.split(' ').skip(2).array_chunks::<2>() {
        let (count, color) = (pair[0].parse::<u32>().unwrap(), pair[1]);
        let index = match color.chars().next().unwrap() {
            'r' => 0,
            'g' => 1,
            'b' => 2,
            _ => unreachable!(),
        };
        rgb[index] = rgb[index].max(count);
    }
    rgb.iter().product()
}
