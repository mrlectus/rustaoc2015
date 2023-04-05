use std::collections::HashSet;

fn part_one() -> usize {
    let file = include_str!("./input").trim();
    let mut freq = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for p in file.chars() {
        match p {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Ops..."),
        }
        freq.insert((x, y));
    }
    freq.len()
}
fn main() {
    println!("{}", part_one());
}
