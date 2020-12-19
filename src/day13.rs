use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::thread;

use crate::get_string;
use crate::{Day, Parts};

#[derive(Clone)]
pub enum Bus {
    Active(u64),
    OutOfService,
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Active(v) => v.to_string(),
                Self::OutOfService => "x".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Bus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::OutOfService),
            vs => vs.parse().map(Self::Active).map_err(|_| ParseError),
        }
    }
}

pub fn find_time_loop(busses: &[Bus], start: u64) -> u64 {
    let delta: u64 = match busses.first() {
        Some(Bus::Active(v)) => *v,
        _ => panic!(),
    };
    let mut timestamp: u64 = start;
    loop {
        if busses.iter().enumerate().all(|(idx, bus)| match bus {
            Bus::Active(n) if (timestamp + idx as u64) % n != 0 => false,
            Bus::Active(n) if (timestamp + idx as u64) % n == 0 => true,
            Bus::OutOfService => true,
            _ => panic!(),
        }) {
            break;
        }
        timestamp += delta;
    }
    timestamp
}

fn find_largest(busses: &[Bus]) -> (usize, u64) {
    match busses.iter().enumerate().max_by_key(|(_, b)| match b {
        Bus::Active(v) => *v,
        Bus::OutOfService => 0,
    }) {
        Some((idx, Bus::Active(v))) => (idx, *v),
        _ => panic!(),
    }
}

pub fn find_time_loop_max(busses: &[Bus], start: u64) -> u64 {
    let (idx, delta) = find_largest(busses);
    let mut timestamp: u64 = start + delta - u64::try_from(idx).unwrap();
    // println!("Starting at {} with delta {}", timestamp, delta);
    loop {
        if busses.iter().enumerate().all(|(idx, bus)| match bus {
            Bus::Active(n) if (timestamp + idx as u64) % n != 0 => false,
            Bus::Active(n) if (timestamp + idx as u64) % n == 0 => true,
            Bus::OutOfService => true,
            _ => panic!(),
        }) {
            break;
        }
        timestamp += delta;
        if timestamp % 1_000_000_000 == 0 {
            // println!("{}", timestamp);
        }
    }
    timestamp
}

pub fn check_times_start_end(busses: &[Bus], start: u64, end: u64, delta: u64) -> Option<u64> {
    let mut timestamp: u64 = start;
    while timestamp < end {
        if busses.iter().enumerate().all(|(idx, bus)| match bus {
            Bus::Active(n) if (timestamp + idx as u64) % n != 0 => false,
            Bus::Active(n) if (timestamp + idx as u64) % n == 0 => true,
            Bus::OutOfService => true,
            _ => panic!(),
        }) {
            return Some(timestamp);
        }
        timestamp += delta;
    }
    None
}

pub fn find_time_thread(busses: &[Bus], start: u64) -> u64 {
    const ITERS_PER_THREAD: u64 = 100_000_000;
    const THREAD_COUNT: u64 = 16;
    let (idx, delta) = find_largest(busses);
    let offset_per_thread = delta * ITERS_PER_THREAD;
    let mut timestamp: u64 = start + delta - u64::try_from(idx).unwrap();
    println!("Starting at {} with delta {}", timestamp, delta);
    let mut threads: VecDeque<_> = (0..THREAD_COUNT)
        .into_iter()
        .map(|idx| {
            let start = timestamp + idx * offset_per_thread;
            let end = start + offset_per_thread;
            let busses: Vec<Bus> = busses.to_vec();
            println!("Spawn thread {} to {}", start, end);
            thread::spawn(move || check_times_start_end(&busses, start, end, delta))
        })
        .collect();
    timestamp += THREAD_COUNT * offset_per_thread;
    loop {
        let thread = threads.pop_front().unwrap();
        match thread.join().unwrap() {
            Some(t) => break t,
            None => {
                let end = timestamp + offset_per_thread;
                let busses: Vec<Bus> = busses.to_vec();
                println!("Spawn thread {} to {}", timestamp, end);
                threads.push_back(thread::spawn(move || {
                    check_times_start_end(&busses, timestamp, end, delta)
                }));
                timestamp = end;
            }
        }
    }
}

fn extended_euclidian(a: i128, b: i128) -> (i128, i128) {
    let (mut old_r, mut curr_r) = (a, b);
    let (mut old_s, mut curr_s) = (1_i128, 0_i128);
    let (mut old_t, mut curr_t) = (0_i128, 1_i128);
    while curr_r != 0 {
        let quotient = old_r / curr_r;
        let new_r = old_r - quotient * curr_r;
        old_r = curr_r;
        curr_r = new_r;
        let new_s = old_s - quotient * curr_s;
        old_s = curr_s;
        curr_s = new_s;
        let new_t = old_t - quotient * curr_t;
        old_t = curr_t;
        curr_t = new_t;
    }
    (old_s, old_t)
}

fn case_of_two_moduli(a: (i128, i128), b: (i128, i128)) -> (i128, i128) {
    let (x, y) = extended_euclidian(a.1, b.1);
    let congruence_solution = b.0 * x * a.1 + a.0 * y * b.1;
    let product = a.1 * b.1;
    (
        ((congruence_solution % product) + product) % product,
        product,
    )
}

pub fn chinese_remainder_busses(busses: &[Bus], _start: u64) -> u64 {
    let congruence_solution = busses
        .iter()
        .enumerate()
        .fold(None, |acc, (idx, bus)| match (acc, bus) {
            (None, Bus::OutOfService) => panic!(),
            (None, Bus::Active(b)) => Some((-i128::try_from(idx).unwrap(), i128::from(*b))),
            (Some(prev), Bus::OutOfService) => Some(prev),
            (Some(prev), Bus::Active(b)) => Some(case_of_two_moduli(
                prev,
                (-i128::try_from(idx).unwrap(), i128::from(*b)),
            )),
        })
        .unwrap();
    u64::try_from(congruence_solution.0).unwrap()
}

pub fn main() -> Day {
    // let target = 939;
    // let busses: Vec<Bus> = "7,13,x,x,59,x,31,19"
    //     .split(",")
    //     .map(|n| n.parse().unwrap())
    //     .collect();

    let target: u64 = get_string("day13.txt")
        .lines()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let busses: Vec<Bus> = get_string("day13.txt")
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // println!("Busses = [{}]", busses.iter().map(|b| b.to_string()).collect::<Vec<String>>().join(","));

    let mut delays: Vec<(u64, u64)> = busses
        .iter()
        .filter_map(|b| match b {
            Bus::Active(v) => Some((*v, (v - target % v))),
            Bus::OutOfService => None,
        })
        .collect();
    // println!("Delays = [{}]", delays.iter().map(|(b, d)| format!("({},{})", b, d)).collect::<Vec<String>>().join(","));
    delays.sort_unstable_by(|(_lhs_busid, lhs_delay), (_rhs_busid, rhs_delay)| {
        lhs_delay.partial_cmp(rhs_delay).unwrap()
    });
    let (busid, delay) = delays.first().unwrap();
    let part1 = busid * delay;
    let part1_display = format!(
        "{} = Bus {} × {} minutes of delay",
        busid * delay,
        busid,
        delay
    );

    let start = 0;
    let time = chinese_remainder_busses(&busses, start);
    let part2_display = format!("{} = is the earliest timestamp", time);

    Day {
        answers: Parts(part1.to_string(), time.to_string()),
        display: Parts(part1_display, part2_display),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "153");
        assert_eq!(day.answers.1, "471793476184394");
    }

    #[test]
    fn test_find_time() {
        let busses: Vec<Bus> = "7,13,x,x,59,x,31,19"
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        for (name, find_time) in [
            (
                "find_time_loop",
                find_time_loop as for<'r> fn(&'r [Bus], u64) -> u64,
            ),
            ("find_time_loop_max", find_time_loop_max),
            ("find_time_thread", find_time_thread),
            ("euclidian_busses", chinese_remainder_busses),
        ]
        .iter()
        {
            println!("{}", name);
            assert_eq!(find_time(&busses, 0), 1068781);
        }
    }

    #[test]
    fn test_case_of_two_moduli() {
        assert_eq!(case_of_two_moduli((0, 3), (3, 4)), (3, 12));
        assert_eq!(case_of_two_moduli((4, 5), (3, 12)), (39, 60));
    }

    #[test]
    fn test_extended_euclidian() {
        assert_eq!(extended_euclidian(12, 42), (-3, 1));
        assert_eq!(extended_euclidian(3, 4), (-1, 1));
    }
}
