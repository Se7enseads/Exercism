pub fn egg_count(display_value: u32) -> usize {
    let bin = format!("{display_value:b}");
    bin
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .into_iter()
        .sum::<u32>()
        .try_into()
        .unwrap()
}
