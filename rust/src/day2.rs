//#[aoc_generator(day2)]
//pub fn input_generator(input: &str) -> Vec<isize> {}

use std::mem;

#[aoc(day2, part1)]
pub fn solve_1(input: &[u8]) -> usize {
    let mut counts: [u8; 26];
    let (mut twos, mut threes) = (0, 0);

    for x in input.split(|&x| x == b'\n') {
        counts = unsafe { mem::zeroed() };

        for &x in x {
            counts[(x - b'a') as usize] += 1;
        }

        twos += counts.iter().any(|&x| x == 2) as usize;
        threes += counts.iter().any(|&x| x == 3) as usize;
    }

    twos * threes
}

#[aoc(day2, part2)]
pub fn solve_2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();

    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if lines[i]
                .chars()
                .zip(lines[j].chars())
                .filter(|&(a, b)| a != b)
                .count()
                == 1
            {
                return lines[i]
                    .chars()
                    .zip(lines[j].chars())
                    .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                    .collect();
            }
        }
    }

    unreachable!();
}

#[aoc(day2, part2, functional)]
pub fn solve_2_functional(input: &str) -> String {
    use itertools::Itertools;

    input
        .lines()
        .cartesian_product(input.lines())
        .filter(|&(a, b)| a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count() == 1)
        .next()
        .map(|(a, b)| {
            a.chars()
                .zip(b.chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = br#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#;
        assert_eq!(solve_1(input), 12);
    }
}
