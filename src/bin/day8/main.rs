use regex::Regex;
fn main() {
    let pattern = Regex::new(r"\\x[0-9a-fA-F]{2}").unwrap();
    let input = include_str!("input");
    let mut sum = 0;
    for line in input.lines() {
        sum += line.escape_default().count() + 2 - line.len();
        let new_line = pattern
            .replace_all(line, "x")
            .replace("\\\"", "x")
            .replace("\\\\", "x")
            .len()
            - 2;
    }
    println!("{sum}");
}
