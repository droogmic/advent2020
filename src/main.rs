use structopt::StructOpt;

use advent2020::get_days;
use advent2020::Day;

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<usize>,

    #[structopt(long)]
    all: bool,

    #[structopt(long)]
    parallel: bool,
}

fn print_day<F>(day: usize, calc: F)
where
    F: FnOnce() -> Day,
{
    println!("Day {}", day);
    let day = calc();
    println!("Part 1: {}", day.display.0);
    println!("Part 2: {}", day.display.1);
    println!();
}

fn main() {
    println!("Advent Of Code 2020");
    println!();

    let args = Cli::from_args();

    if args.all {
        for (idx, day) in get_days().into_iter().enumerate() {
            print_day(idx + 1, day);
        }
    }

    if args.parallel {
        let threads: Vec<_> = get_days()
            .into_iter()
            .enumerate()
            .map(|(idx, calc)| {
                println!("Spawn day {}", idx + 1);
                std::thread::spawn(calc)
            })
            .collect();
        std::thread::yield_now();
        // std::thread::sleep(std::time::Duration::from_secs(1));
        println!();
        for (idx, thread) in threads.into_iter().enumerate() {
            print_day(idx + 1, || thread.join().unwrap());
        }
    }

    if !(args.all || args.parallel) {
        let days = get_days();
        match args.puzzle {
            None => print_day(days.len(), *days.last().unwrap()),
            Some(n) => print_day(n, *days.get(n - 1).expect("invalid day")),
        }
    }
}
