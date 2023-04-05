fn main() {
    let token = "iwrupvqb";
    let mut start = 0;
    loop {
        let hash = format!("{}{}", token, start);
        let digest = md5::compute(hash);
        if format!("{:x}", digest)[0..6] == *"000000" {
            println!("{}", start);
            break;
        }
        start += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    }
}
