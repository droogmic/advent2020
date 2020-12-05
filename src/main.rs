use structopt::StructOpt;

use advent2020::day1;

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<usize>,
}

fn main() {
    println!("Advent Of Code 2020");

    let day_exec: Vec<Box<dyn Fn()>> = vec![Box::new(day1::main)];

    let args = Cli::from_args();

    println!("Day {}", args.puzzle.unwrap_or(day_exec.len()));
    match args.puzzle {
        None => day_exec.last().unwrap()(),
        Some(n) => day_exec.get(n - 1).expect("invalid day")(),
    }
}
