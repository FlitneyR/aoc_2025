use aoc_2025_proc_macros::*;
use aoc_2025_common::*;

#[derive(Clone, Copy, Debug)]
struct ID(u64);

#[derive(Debug, FromRegexCaptures)]
struct IDRange {
    start: u64,
    end: u64,
}

impl ID {
    pub fn is_valid(&self) -> bool {
        let str = self.0.to_string();

        #[allow(unused)]
        #[cfg(feature = "part1")]
        let result = {
            str[..str.len() / 2] == str[str.len() / 2..]
        };

        #[cfg(feature = "part2")]
        let result = {
            // unfortunately, regex::Regex doesn't support backreferences, so we can't do this oneliner
            // regex::Regex::new(r"^[0-9]\1+$").unwrap().find(&str).is_none()

            for length in 1..=str.len() / 2 {
                if str.len() % length == 0 {
                    let iter_1 = str.as_bytes().chunks(length);
                    let iter_2 = iter_1.clone().skip(1);
                    let mut iter = iter_1.zip(iter_2);

                    if iter.all(|(pattern_1, pattern_2)| pattern_1 == pattern_2) {
                        return false;
                    }
                }
            }

            true
        };

        result
    }
}

impl IDRange {
    pub fn sum_invalid_ids(&self) -> u64 {
        (self.start..=self.end)
            .filter(|num| !ID(*num).is_valid())
            .sum()
    }
}

fn main() {
    let regex = regex::Regex::new("(?<start>[0-9]+)-(?<end>[0-9]+)").unwrap();
    let total: u64 = aoc_2025_common::get_input().unwrap()
        .iter_by_regex(&regex)
        .map(|range: IDRange| range.sum_invalid_ids())
        .sum();

    println!("{total}");
}
