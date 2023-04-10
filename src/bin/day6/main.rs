use std::fs::read_to_string;

#[derive(Debug)]
pub struct FireHazard {
    pub action: String,
    pub cord1: (i32, i32),
    pub cord2: (i32, i32),
}

impl FireHazard {
    pub fn parse_hazard(line: &str) -> Vec<FireHazard> {
        let mut hazard = Vec::new();
        let line = read_to_string(line).unwrap();
        for action in line.lines() {
            let parse_line = action.split(' ').collect::<Vec<_>>();
            let mut action = String::new();
            let mut cord1 = (0, 0);
            let mut cord2 = (0, 0);
            if parse_line[0] == "turn" && parse_line[1] == "on" {
                action = "turn_on".to_string();
                let c1 = parse_line[2]
                    .split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                cord1.0 = c1[0];
                cord1.1 = c1[1];
                let c2 = parse_line[4]
                    .split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                cord2.0 = c2[0];
                cord2.1 = c2[1];
            }
            if parse_line[0] == "turn" && parse_line[1] == "off" {
                action = "turn_off".to_string();
                let c1 = parse_line[2]
                    .split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                cord1.0 = c1[0];
                cord1.1 = c1[1];
                let c2 = parse_line[4]
                    .split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                cord2.0 = c2[0];
                cord2.1 = c2[1];
            }
            if parse_line[0] == "toggle" {
                action = "toggle".to_string();
                let c1 = parse_line[1]
                    .split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                cord1.0 = c1[0];
                cord1.1 = c1[1];
                let c2 = parse_line[3]
                    .split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                cord2.0 = c2[0];
                cord2.1 = c2[1];
            }
            hazard.push(FireHazard {
                action,
                cord1,
                cord2,
            })
        }
        hazard
    }
}

fn main() {
    let mut v1 = vec![vec![0; 1000]; 1000];
    let hazard: Vec<_> = FireHazard::parse_hazard("src/bin/day6/input");
    for x in hazard.iter() {
        match x.action.as_str() {
            "turn_on" => {
                for i in x.cord1.0..=x.cord2.0 {
                    for j in x.cord1.1..=x.cord2.1 {
                        v1[i as usize][j as usize] += 1;
                    }
                }
            }
            "turn_off" => {
                for i in x.cord1.0..=x.cord2.0 {
                    for j in x.cord1.1..=x.cord2.1 {
                        if v1[i as usize][j as usize] > 0 {
                            v1[i as usize][j as usize] -= 1;
                        }
                    }
                }
            }
            "toggle" => {
                for i in x.cord1.0..=x.cord2.0 {
                    for j in x.cord1.1..=x.cord2.1 {
                        v1[i as usize][j as usize] += 2;
                    }
                }
            }
            _ => {}
        }
    }
    let mut count = 0;
    for x in v1.iter() {
        for y in x.iter() {
            if *y >= 1 {
                count += y;
            }
        }
    }
    println!("{}", count);
}
#[cfg(test)]
mod test {
    #[test]
    fn test_hazard() {
        let mut count_all = 0;
        let mut v1 = vec![vec![0; 100]; 100];
        for x in 0..=0 {
            for y in 0..=0 {
                v1[x][y] += 1;
            }
        }
        for x in v1.iter() {
            for y in x.iter() {
                count_all += 1;
                print!("{}", *y);
            }
            println!();
        }
        println!("count_all{}", count_all);
    }
}
