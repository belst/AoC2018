fn count_dbl_and_trpl(input: &str) -> (bool, bool) {
    let mut c = std::collections::HashMap::new();
    for ch in input.chars() {
        *c.entry(ch).or_insert(0) += 1;
    }
    (c.values().any(|&n| n == 2), c.values().any(|&n| n == 3))
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let (dbls, trpls) = input
        .lines()
        .map(count_dbl_and_trpl)
        .fold((0, 0), |(d, t), (c, l)| (d + c as usize, t + l as usize));

    dbls * trpls
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let input = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

    for i in 0..input.len() {
        'cont: for j in (i + 1)..input.len() {
            let mut matching = Vec::new();
            let mut misses = 0;
            for n in 0..input[i].len() {
                if input[i][n] == input[j][n] {
                    matching.push(input[i][n]);
                } else {
                    misses += 1;
                    if misses > 1 {
                        continue 'cont;
                    }
                }
            }
            return String::from_utf8_lossy(&matching[..]).into();
        }
    }

    "Not found".into()
}
