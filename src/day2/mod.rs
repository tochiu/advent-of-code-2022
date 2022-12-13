pub fn solution1(path: &str) -> impl ToString {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::chars)
        .map(|mut chars| {
            let opp = match chars.next().unwrap() {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => unreachable!()
            };
            let (you, score) = match chars.skip(1).next().unwrap() {
                'X' => (0, 1),
                'Y' => (1, 2),
                'Z' => (2, 3),
                _ => unreachable!()
            };

            score + if you == opp {
                3
            } else if you == (opp + 1) % 3 {
                6
            } else {
                0
            }
        })
        .sum::<u32>()
}

pub fn solution2(path: &str) -> impl ToString {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::chars)
        .map(|mut chars| {
            let opp = match chars.next().unwrap() {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => unreachable!()
            };
            match chars.skip(1).next().unwrap() {
                'X' => 1 + (opp + 2) % 3,
                'Y' => 4 + opp,
                'Z' => 7 + (opp + 1) % 3,
                _ => unreachable!()
            }
        })
        .sum::<u32>()
}