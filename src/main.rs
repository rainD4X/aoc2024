mod day01;
mod day02;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let days: Vec<Vec<Box<dyn Fn()>>> = vec![
        vec![
            Box::new(day01::part_one),
            Box::new(day01::part_two),
        ],
        vec![
            Box::new(day02::part_one),
            Box::new(day02::part_two),
        ],
    ];

    // The first argument is the program name
    println!("Program name: {}", args[0]);

    if args.len() != 3 {
        println!("Usage : {} NDAY NPART", args[0]);
        std::process::exit(1);
    }
    let day = args[1].parse::<usize>().unwrap_or_default() - 1;
    let part = args[2].parse::<usize>().unwrap_or_default() - 1;

    if day < days.len()
    {
        if part < days[day].len() {
            days[day][part]();
        }
        else {
            println!("Day {} Part {} not found", day + 1, part + 1);
        }
    }
    else  {
        println!("Day {} not found", day+ 1);
    }
}
