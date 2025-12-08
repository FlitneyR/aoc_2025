use std::str::FromStr;

#[derive(Debug)]
struct BatteryBank {
    batteries: Box<[u8]>
}

impl FromStr for BatteryBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{ batteries: s.chars().map(|c| if c.is_numeric() { Some(c as u8 - '0' as u8) } else { None } ).flatten().collect() })
    }
}

impl std::fmt::Display for BatteryBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for battery in &self.batteries {
            write!(f, "{battery}")?
        }

        write!(f, "")
    }
}

impl BatteryBank {
    pub fn largest_sequential_combination(&self, count: usize) -> usize {
        let mut slice: &[u8] = &self.batteries;
        let mut acc = 0usize;

        for digit_index in 1..=count {
            let remaining_digits = count - digit_index;

            let (next_index, next_largest_number) = slice[..slice.len() - remaining_digits]
                .iter().enumerate().rev() // rev to get the _first_ max, not the last
                .max_by_key(|&(_index, &value)| value)
                .unwrap();
    
            slice = &slice[next_index + 1..];
            acc = 10 * acc + *next_largest_number as usize
        }

        acc
    }
}

fn main() {
    let regex = regex::Regex::new("[0-9]+").unwrap();

    let sum: usize = aoc_2025_common::get_input().unwrap().lines()
        .map(|line| regex.find(&line).unwrap().as_str().parse().unwrap())
        .map(|bank: BatteryBank| {
            #[cfg(feature = "part1")]
            return bank.largest_sequential_combination(2);

            #[allow(unreachable_code)] // this _is_ actually reachable, so long as part1 and part2 are not both used (why would you do that?)
            #[cfg(feature = "part2")]
            return bank.largest_sequential_combination(12);
        })
        .sum();

    println!("{sum}")
}
