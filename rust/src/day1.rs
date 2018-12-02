use hashbrown::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_1(input: &[isize]) -> isize {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_2(input: &[isize]) -> isize {
    let mut set = HashSet::new();
    input
        .iter()
        .cycle()
        .scan(0, |c, &x| {
            *c += x;
            Some(*c)
        })
        .filter(|&x| !set.insert(x))
        .next()
        .unwrap()
}
