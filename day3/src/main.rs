use std::{collections::HashMap, iter::once};

fn main() {
    let input = include_str!("../input");
    let part1 = part1(input);
    let part2 = part2(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (line_idx, line) in grid.iter().enumerate() {
        // Check line for numbers
        let mut chars = line.iter().enumerate();
        while let Some((char_idx, c)) = chars.next() {
            if c.is_digit(10) {
                // Parse number
                let (len, value) = get_number(&line[char_idx..]);
                advance_by(&mut chars, len - 1);

                // Check all adjacent squares for engine parts
                let mut adjacent = adjacent(line_idx, char_idx, len);
                let engine_part_found = adjacent.any(|(line_idx, idx)| {
                    if let Some(c) = grid.get(line_idx).and_then(|line| line.get(idx)) {
                        *c != '.' && !c.is_numeric()
                    } else {
                        false
                    }
                });
                if engine_part_found {
                    sum += value;
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    struct Gear {
        count: u32,
        product: u32,
    }
    let mut gears = HashMap::<(usize, usize), Gear>::new();
    for (line_idx, line) in grid.iter().enumerate() {
        // Check line for numbers
        let mut chars = line.iter().enumerate();
        while let Some((char_idx, c)) = chars.next() {
            if c.is_digit(10) {
                // Parse number
                let (len, value) = get_number(&line[char_idx..]);
                advance_by(&mut chars, len - 1);

                // Check all adjacent squares for engine parts
                let adjacent = adjacent(line_idx, char_idx, len);
                for (line, idx) in adjacent {
                    if let Some(c) = grid.get(line).and_then(|line| line.get(idx)) {
                        if *c == '*' {
                            let gear = gears.entry((line, idx)).or_insert(Gear {
                                count: 0,
                                product: 1,
                            });
                            if gear.count < 2 {
                                gear.product *= value;
                            }
                            gear.count += 1;
                        }
                    }
                }
            }
        }
    }

    gears
        .values()
        .filter(|gear| gear.count == 2)
        .map(|gear| gear.product)
        .sum()
}

fn get_number(chars: &[char]) -> (usize, u32) {
    chars
        .iter()
        .take_while(|c| c.is_digit(10))
        .fold((0, 0), |(len, value), c| {
            (len + 1, value * 10 + c.to_digit(10).unwrap())
        })
}

fn adjacent(line_idx: usize, start: usize, len: usize) -> impl Iterator<Item = (usize, usize)> {
    let (line_idx, start, len) = (line_idx as isize, start as isize, len as isize);
    let end = start + len - 1;

    let above = line_idx - 1;
    let below = line_idx + 1;
    let left = start - 1;
    let right = end + 1;

    (above..=below).flat_map(move |line| {
        (left..=right).filter_map(move |idx| {
            let is_number = line == line_idx && idx >= start && idx <= end;
            let out_of_bounds = line < 0 || idx < 0;
            let is_adjacent = !out_of_bounds && !is_number;

            (is_adjacent).then(|| (line as usize, idx as usize))
        })
    })
}

fn advance_by<T>(iter: &mut impl Iterator<Item = T>, n: usize) {
    for _ in 0..n {
        iter.next();
    }
}
