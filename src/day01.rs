use core::str;

const INPUT: &str = include_str!("../inputs/day01");

pub fn part_one() {
    let mut left: Vec<usize> = Vec::with_capacity(1000);
    let mut right: Vec<usize> = Vec::with_capacity(1000);

    INPUT.split_terminator('\n').for_each(|line| {
        let mut iter = line.split_ascii_whitespace();
        let (l, r) = (iter.next().unwrap(), iter.next().unwrap());

        left.push(l.parse::<usize>().unwrap());
        right.push(r.parse::<usize>().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    let total_disance = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<usize>();

    println!("Total Distance : {}", total_disance);
}

pub fn part_two() {
    let mut left: Vec<usize> = Vec::with_capacity(1000);
    let mut right: Vec<usize> = Vec::with_capacity(1000);

    INPUT.split_terminator('\n').for_each(|line| {
        let mut iter = line.split_ascii_whitespace();
        let (l, r) = (iter.next().unwrap(), iter.next().unwrap());

        left.push(l.parse::<usize>().unwrap());
        right.push(r.parse::<usize>().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    let mut riter = right.iter();
    let mut r = riter.next().unwrap();

    let similarity = left
        .iter()
        .map(|l| {
            let mut count = 0;
            loop {
                if r == l {
                    count += 1;
                } else if r > l {
                    break;
                }
                if let Some(next) = riter.next() {
                    r = next
                } else {
                    break;
                }
            }
            l * count
        })
        .sum::<usize>();
    println!("Similarity: {}", similarity)
}
