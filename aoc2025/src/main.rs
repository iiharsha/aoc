use aoclib::{Runner, Selector};

mod day_1;
mod day_2;
/*
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
*/

use day_1::*;
use day_2::*;
/*
use day_3::*;
use day_4::*;
use day_5::*;
use day_6::*;
use day_7::*;
use day_8::*;
use day_8::*;
use day_9::*;
use day_10::*;
use day_11::*;
use day_12::*;
*/

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        run_2025(Selector::Last);
    } else if let Ok(day) = args[1].parse::<usize>() {
        run_2025(Selector::One(day));
    } else if args[1] == "-a" || args[1] == "--all" {
        run_2025(Selector::All);
    }
}

fn run_2025(which: Selector) {
    let mut day01 = Day1::new();
    let mut day02 = Day2::new();

    let mut days: Vec<&mut dyn Runner> = vec![&mut day01, &mut day02];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            aoclib::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                aoclib::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            aoclib::run_solution(*d);
        }
    }
}
