pub fn solution1(path: &str) -> impl ToString {
    std::fs::read_to_string(path)
        .unwrap()
        .split(&crate::config::LINE_ENDING.repeat(2))
        .map(|elf| {
            elf.lines()
                .map(str::parse::<u32>)
                .filter_map(Result::ok)
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn solution2(path: &str) -> impl ToString {
    std::fs::read_to_string(path)
        .unwrap()
        .split(&crate::config::LINE_ENDING.repeat(2))
        .map(|elf| {
            elf.lines()
                .map(str::parse::<u32>)
                .filter_map(Result::ok)
                .sum::<u32>()
        })
        .fold([0, 0, 0], |mut top_calories, calories| {
            if calories > top_calories[2] {
                let insertion_point = top_calories
                    .binary_search_by(|x| calories.cmp(x))
                    .unwrap_or_else(|x| x);
                let copy_range = insertion_point..top_calories.len() - 1;

                top_calories.copy_within(copy_range, insertion_point + 1);
                top_calories[insertion_point] = calories;
            }
            top_calories
        })
        .iter()
        .sum::<u32>()
}