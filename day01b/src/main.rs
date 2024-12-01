use core::str;

fn main() {
    let input = str::from_utf8(include_bytes!("../../inputs/day01")).unwrap();
    let mut left: Vec<usize> = Vec::with_capacity(1000);
    let mut right: Vec<usize> = Vec::with_capacity(1000);

    input.split_terminator('\n').for_each(|line| {
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
        }).sum::<usize>();
    println!("Similarity: {}", similarity)
}
