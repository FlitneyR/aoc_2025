use aoc_2025_proc_macros::*;
use aoc_2025_common::*;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Direction { Left = -1, Right = 1 }

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().ok_or(())? { 'L' => Ok(Self::Left), 'R' => Ok(Self::Right), _ => Err(()) }
    }
}

#[derive(Debug, FromRegexCaptures)]
struct Command {
    direction: Direction,
    count: u32,
}

impl Command {
    pub fn get_delta(&self) -> i32 { self.count as i32 * self.direction as i32 }
}

struct AnswerAccumulator {
    zero_count: u32
}

impl AnswerAccumulator {
    pub fn new() -> Self { Self { zero_count: 0 } }
    pub fn increment(&mut self) { self.add(1) }
    pub fn add(&mut self, count: u32) { self.zero_count += count }
    pub fn get(&self) -> u32 { self.zero_count }
}

struct DialState {
    value: i32
}

impl DialState {
    const DIAL_INITIAL: i32 = 50;
    const DIAL_MAX: i32 = 100;

    pub fn new() -> Self { Self { value: Self::DIAL_INITIAL } }

    #[allow(unused)]
    pub fn rotate_part1(&mut self, answer_accumulator: &mut AnswerAccumulator, command: &Command) { 
        self.value = (self.value + command.get_delta() + Self::DIAL_MAX) % Self::DIAL_MAX;
        if self.value == 0 { answer_accumulator.increment() }
    }

    #[allow(unused)]
    pub fn rotate_part2(&mut self, answer_accumulator: &mut AnswerAccumulator, command: &Command) { 
        answer_accumulator.add(self.count_zero_passes(command.get_delta()));
        self.value = (self.value + (command.get_delta() % Self::DIAL_MAX) + Self::DIAL_MAX) % Self::DIAL_MAX;
    }

    fn count_zero_passes(&self, delta: i32) -> u32 {
        //            | add the delta              | map to 50..100                              | remove double counting from going down from 0
        (((self.value + delta - Self::DIAL_MAX / 2).abs() + Self::DIAL_MAX / 2) / Self::DIAL_MAX - (delta < 0 && self.value == 0) as i32).max(0) as u32
        //                    | map to -50..50                                  | count times past 100                                   | this might be negative, which should map to 0
    }
}

fn main() {
    let mut answer_accumulator = AnswerAccumulator::new();
    let mut dial = DialState::new();
    let regex = regex::Regex::new("(?<direction>[LR])(?<count>[0-9]+)").unwrap();

    get_input().unwrap().lines()
        .for_each(|line| {
            let cmd: Command = line.as_str().iter_by_regex(&regex).next().unwrap();

            #[cfg(feature = "part1")]
            dial.rotate_part1(&mut answer_accumulator, &cmd);

            #[cfg(feature = "part2")]
            dial.rotate_part2(&mut answer_accumulator, &cmd);
        });

    println!("{}", answer_accumulator.get());
}
