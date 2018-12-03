pub struct Rect {
    id: usize,
    pos: (usize, usize),
    size: (usize, usize),
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Rect> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut tmp = l.split_whitespace();
            let id = tmp.next().unwrap();
            let id = id.trim_start_matches('#').parse().unwrap();
            tmp.next().unwrap();
            let pos = tmp.next().unwrap();
            let mut pos = pos
                .split(',')
                .map(|n| n.trim_end_matches(':').parse().unwrap());
            let size = tmp.next().unwrap();
            let mut size = size.split('x').map(|s| s.parse().unwrap());

            Rect {
                id: id,
                pos: (pos.next().unwrap(), pos.next().unwrap()),
                size: (size.next().unwrap(), size.next().unwrap()),
            }
        }).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Rect]) -> usize {
    let mut occ = std::collections::HashMap::new();

    for i in 0..input.len() {
        let c = &input[i];
        for x in c.pos.0..(c.size.0 + c.pos.0) {
            for y in c.pos.1..(c.size.1 + c.pos.1) {
                *occ.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    occ.values().filter(|&&c| c > 1).count()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Rect]) -> usize {
    let mut occ = std::collections::HashMap::new();

    for i in 0..input.len() {
        let c = &input[i];
        for x in c.pos.0..(c.size.0 + c.pos.0) {
            for y in c.pos.1..(c.size.1 + c.pos.1) {
                *occ.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    input
        .iter()
        .find(|&r| {
            let mut all = true;
            for x in r.pos.0..r.size.0 + r.pos.0 {
                for y in r.pos.1..r.size.1 + r.pos.1 {
                    all = all && occ
                        .get(&(x, y))
                        .filter(|&&v| v == 1)
                        .map(|_| true)
                        .unwrap_or(false);
                }
            }
            all
        }).unwrap()
        .id
}
