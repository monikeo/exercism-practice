const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let position = CHILDREN.iter().position(|&object| object == student);
    if let Some(position) = position {
        let position = position * 2;
        diagram
            .split('\n')
            .flat_map(|line| {
                line[position..=position + 1].chars().map(|ch| match ch {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => unreachable!(),
                })
            })
            .collect()
    } else {
        Vec::new()
    }
}
