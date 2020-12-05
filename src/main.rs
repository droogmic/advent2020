use structopt::StructOpt;

use advent2020::get_days;

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<usize>,
}

fn main() {
    println!("Advent Of Code 2020");

    let days = get_days();

    let args = Cli::from_args();

    println!("Day {}", args.puzzle.unwrap_or(days.len()));
    match args.puzzle {
        None => days.last().unwrap()(),
        Some(n) => days.get(n - 1).expect("invalid day")(),
    }
}
