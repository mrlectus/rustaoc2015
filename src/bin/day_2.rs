#[allow(unused)]
fn surface_area(dimention: &[i32]) -> i32 {
    (2 * dimention[0] * dimention[1])
        + (2 * dimention[1] * dimention[2])
        + (2 * dimention[0] * dimention[2])
        + i32::min(
            (dimention[0] * dimention[1]),
            i32::min((dimention[1] * dimention[2]), (dimention[0] * dimention[2])),
        )
}

#[allow(unused)]
fn first_part() {
    let v1 = include_str!("./day_2");
    let group = v1
        .trim()
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|inner| {
            return inner
                .split('x')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();
    let sum = group
        .iter()
        .map(|get_sum| surface_area(get_sum))
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();
    println!("{sum:?}")
}

#[allow(unused)]
fn second_part() {
    let v1 = include_str!("./day_2");
    let mut groups = v1
        .trim()
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|group| {
            group
                .split('x')
                .map(|inner| inner.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cal_ribbon = groups
        .iter_mut()
        .map(|mut dim| ribbon(dim))
        .collect::<Vec<_>>();
    println!("{:#?} feets", cal_ribbon.iter().sum::<i32>());
}

fn ribbon(list: &mut [i32]) -> i32 {
    list.sort();
    let multiples = list[0] * list[1] * list[2];
    let wrap = list[0] * 2 + list[1] * 2;
    multiples + wrap
}

fn main() {
    second_part();
}
