pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rows: Vec<&str> = diagram.lines().collect();
    
    let first_row: Vec<char> = rows[0].chars().collect();
    let first: Vec<_> = first_row.chunks(2).collect();
    
    let second_row: Vec<char> = rows[1].chars().collect();
    let second: Vec<_> = second_row.chunks(2).collect();

    let final_garden: Vec<Vec<&'static str>> = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a.iter().chain(b.iter()).copied().collect::<Vec<_>>())
        .map(|plants| plants.iter().map(|plant| {
            match plant {
                'V' => "violets",
                'C' => "clover",
                'R' => "radishes",
                'G' => "grass",
                _ => unreachable!()
            }
        }).collect::<Vec<&'static str>>())
        .collect();
    dbg!(&final_garden);

    let students = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];

    let idx = students.into_iter().position(|std| std == student).unwrap();

    final_garden[idx].clone()
}
