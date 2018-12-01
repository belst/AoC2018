#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    input.lines().map(|x| x.parse::<i64>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let mut seen = std::collections::HashSet::new();

    let mut it = input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .cycle()
        .scan(0, |a, c| {
            *a += c;
            Some(*a)
        });

    loop {
        let v = it.next().unwrap();
        if seen.contains(&v) {
            break v;
        }
        seen.insert(v);
    }
}
