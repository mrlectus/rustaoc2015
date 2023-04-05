use std::collections::HashSet;

pub fn contains_vowel(word: &str) -> Option<i32> {
    let vowel = "aeiou";
    let mut count = 0;
    let mut vowel_map = HashSet::new();
    for x in vowel.chars() {
        vowel_map.insert(x);
    }

    for x in word.chars() {
        if vowel_map.contains(&x) {
            count += 1;
        }
    }
    Some(count)
}

pub fn not_contain_string(word: &str) -> Option<&str> {
    let filter_string = HashSet::from(["ab", "cd", "pq", "xy"]);
    for x in 0..word.len() - 1 {
        let word_slice = &word[x..x + 2];
        if filter_string.contains(word_slice) {
            return Some(word_slice);
        }
    }
    None
}
pub fn twice_in_row(word: &str) -> Option<char> {
    let word = word.as_bytes();
    for x in 0..word.len() - 1 {
        if word[x] == word[x + 1] {
            return Some(word[x] as char);
        }
    }
    None
}

pub fn twice_pair_letter(word: &str) -> Option<&str> {
    for x in 0..word.len() - 1 {
        let pairs = &word[x..x + 2];
        for j in (x + 2)..word.len() - 1 {
            if &word[j..j + 2] == pairs {
                return Some(pairs);
            }
        }
    }
    None
}

pub fn one_lettet_repete(word: &str) -> Option<char> {
    let word = word.as_bytes();
    for i in 0..word.len() - 2 {
        if word[i] == word[i + 2] {
            return Some(word[i + 1] as char);
        }
    }
    None
}

fn main() {
    let input = include_str!("input");
    let mut count_nice = 0;
    let mut count_nice_2 = 0;
    for line in input.lines() {
        if contains_vowel(&line).unwrap() >= 3
            && twice_in_row(&line).is_some()
            && not_contain_string(&line).is_none()
        {
            count_nice += 1;
        }
    }
    for line in input.lines() {
        if twice_pair_letter(&line).is_some() && one_lettet_repete(&line).is_some() {
            count_nice_2 += 1;
        }
    }
    println!("{count_nice} {count_nice_2}");
}

#[cfg(test)]
mod test {
    use crate::{
        contains_vowel, not_contain_string, one_lettet_repete, twice_in_row, twice_pair_letter,
    };

    #[test]
    fn test_contains_three_vowel() {
        assert_eq!(contains_vowel("xazegov"), Some(3));
        assert_eq!(contains_vowel("aeiou"), Some(5));
        assert_eq!(contains_vowel("aeiouaeiouaeiou"), Some(15));
    }
    #[test]
    fn test_twice_in_row() {
        assert_eq!(twice_in_row("abcdde"), Some('d'));
        assert_eq!(twice_in_row("aabbccdd"), Some('a'));
        assert_eq!(twice_in_row("abcdefghi"), None);
        assert_eq!(twice_in_row("abcdefghijj"), Some('j'));
    }
    #[test]
    fn test_not_contains_word() {
        assert_eq!(not_contain_string("haegwjzuvuyypxyu"), Some("xy"));
    }
    #[test]
    fn test_twice_pair_letter() {
        assert_eq!(twice_pair_letter("aaab"), None);
        assert_eq!(twice_pair_letter("xyxy"), Some("xy"));
        assert_eq!(twice_pair_letter("aabcdefgaa"), Some("aa"));
        assert_eq!(twice_pair_letter("ieodomkazucvgmuy"), None);
        assert_eq!(twice_pair_letter("xxyxx"), Some("xx"));
    }
    #[test]
    fn test_one_letter_repete() {
        assert_eq!(one_lettet_repete("xyx"), Some('y'));
        assert_eq!(one_lettet_repete("abcdefeghi"), Some('f'));
        assert_eq!(one_lettet_repete("aaa"), Some('a'));
        assert_eq!(one_lettet_repete("aaba"), Some('b'));
        assert_eq!(one_lettet_repete("xxyxx"), Some('y'));
    }

    #[test]
    fn test_both() {
        assert!(
            one_lettet_repete("qjhvhtzxzqqjkmpb").is_some()
                && twice_pair_letter("qjhvhtzxzqqjkmpb").is_some()
        );
        assert!(one_lettet_repete("xxyxx").is_some() && twice_pair_letter("xxyxx").is_some());
    }
}
