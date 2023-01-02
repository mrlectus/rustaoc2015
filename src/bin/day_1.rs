fn main() {
    let dfile = include_str!("./day_1");
    let mut sum = 0;
    for (x, y) in dfile.as_bytes().iter().enumerate() {
        match y {
            40 => sum += 1,
            41 => sum -= 1,
            _ => sum += 0,
        }
        if sum == -1 {
            println!("{x}");
            break;
        }
    }
}
