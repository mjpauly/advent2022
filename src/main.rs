use std::env;
use std::fs;

mod day01; mod day02; mod day03; mod day04;

fn main() {
    let day = get_day();
    let runners: Vec<fn(&str)> =
        vec![day01::run, day02::run, day03::run, day04::run];
    if day == 0 {  // run all days
        for i in 1..runners.len()+1 {
            run_day(i, &runners);
        }
    } else {
        run_day(day, &runners);
    }
}

fn run_day(day: usize, runners: &Vec<fn(&str)>) {
    /* Run a day given the day number and the vector of run functions. */
    println!("\nDay {}:", day);
    let input = fs::read_to_string(format!("inputs/day{:02}.txt", day))
                        .expect("Error: Can't open file.");
    let run: fn(&str) = runners[day-1];
    run(&input);
}

fn get_day() -> usize {
    /* Parse command line argument for the day number.
     * 0 is used to indicate that all days should be run.
     */
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: please input one number as an argument");
    }
    let day: usize = args[1]
        .trim()
        .parse()
        .expect("Error: argument must be a number");
    day
}
