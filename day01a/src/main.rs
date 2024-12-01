use core::str;

fn main() {
    let input = str::from_utf8(include_bytes!("../../inputs/day01a")).unwrap();
    let mut left : Vec<usize> = Vec::with_capacity(1000);
    let mut right : Vec<usize> = Vec::with_capacity(1000);

    input.split_terminator('\n')
        .for_each( | line | {
            let mut iter = line.split_ascii_whitespace();
            let (l, r) = (iter.next().unwrap(), iter.next().unwrap());

            left.push(l.parse::<usize>().unwrap());
            right.push(r.parse::<usize>().unwrap());
        });

    left.sort_unstable();
    right.sort_unstable();

    let total_disance = left.iter().zip(right.iter()).map(| (l, r) | l.abs_diff(*r)).sum::<usize>();

    println!("Total Distance : {}", total_disance)
}
