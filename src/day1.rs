#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut seen = std::collections::HashSet::new();

    input
        .iter()
        .cycle()
        .scan(0, |a, c| {
            *a += c;
            Some(*a)
        }).find(|c| !seen.insert(*c))
        .unwrap()
}
