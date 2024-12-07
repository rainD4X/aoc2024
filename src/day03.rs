use regex::{self, Regex};

const INPUT: &str = include_str!("../inputs/day03");

pub fn part_one() {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let result = re
        .captures_iter(INPUT)
        .map(|c| {
            c.get(0)
                .unwrap()
                .as_str()
                .replace(&['m', 'u', 'l', '(', ')'], "")
        })
        .map(|s| {
            let (lhs, rhs) = s.split_once(',').unwrap();
            lhs.parse::<usize>().unwrap() * rhs.parse::<usize>().unwrap()
        })
        .sum::<usize>();
    println!("Result: {}", result);
}

pub fn part_two() {
    let mut mul_enabled = true;
    let mut result = 0;
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(INPUT)
        .map(|c| {
            c.get(0)
                .unwrap()
                .as_str()
                .replace(&['m', 'u', 'l', '(', ')'], "")
        })
        .for_each(|s| {
            if s == "don't" {
                mul_enabled = false;
            } else if s == "do" {
                mul_enabled = true;
            } else if mul_enabled {
                let (lhs, rhs) = s.split_once(',').unwrap();
                result += lhs.parse::<usize>().unwrap() * rhs.parse::<usize>().unwrap();
            }
        });
    println!("Result: {}", result);
}
