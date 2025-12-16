use std::collections::HashSet;
use aoc_2025_common::*;

#[derive(Clone, Debug)]
#[allow(unused)]
struct Machine {
    lights_bitset: u16,
    buttons: Box<[u16]>,
    joltage_requirements: Box<[usize]>,
}

impl FromRegexCaptures for Machine {
    fn from_regex_captures(captures: &regex::Captures) -> Result<Self, FromRegexCapturesError> {
        let lights_str = captures.name("lights")
            .ok_or(FromRegexCapturesError::MissingField("lights"))?
            .as_str();

        let buttons_str = captures.name("buttons")
            .ok_or(FromRegexCapturesError::MissingField("buttons"))?
            .as_str();

        let joltage_requirements_str = captures.name("joltage_requirements")
            .ok_or(FromRegexCapturesError::MissingField("joltage_requirements"))?
            .as_str();
        
        let bracketed_regex = regex::Regex::new(r"\([^\)]*\)").unwrap();
        let num_regex = regex::Regex::new(r"[0-9]+").unwrap();
        
        let lights_bitset = lights_str.char_indices()
                .map(|(index, c)| ((c == '#') as u16) << index)
                .fold(0u16, |acc, b| acc | b);

        let buttons = bracketed_regex
                .find_iter(buttons_str)
                .map(|s| num_regex
                    .find_iter(s.as_str())
                    .flat_map(|s| s.as_str().parse())
                    .map(|index: u16| 1 << index)
                    .fold(0u16, |acc, b| acc | b))
                .collect();

        let joltage_requirements: Box<[_]> = num_regex
                .find_iter(joltage_requirements_str)
                .flat_map(|s| s.as_str().parse())
                .collect();

        if lights_str.chars().count() != joltage_requirements.len() {
            return Err(FromRegexCapturesError::Malformed("Number of lights does not equal number of joltage requirements"));
        }

        let result = Self { lights_bitset, buttons, joltage_requirements };

        #[cfg(feature = "verbose")]
        eprintln!("\nParsing Machine:{:?}\nlights: {lights_str:?}\nbuttons: {buttons_str:?}\njoltage: {joltage_requirements_str}\n{result:?}",
            captures.get_match().as_str(),
        );

        Ok(result)
    }
}

impl Machine {
    pub fn a_star(&self, bitset: u16) -> Option<Vec<u16>> {
        #[cfg(feature = "verbose")]
        eprintln!("Trying to solve a_star({bitset}({bitset:#b}), {:?})", self.buttons);

        let mut visited_nodes = HashSet::<u16>::new();
        let mut next_nodes = Vec::<(u16, Vec<u16>)>::new();
        next_nodes.push((bitset, Vec::new()));

        loop {
            // #[cfg(feature = "verbose")]
            // eprintln!("Visited nodes: {visited_nodes:?}\nNext nodes: {next_nodes:?}");

            let (node, path) = next_nodes.pop()?;

            // #[cfg(feature = "verbose")] eprintln!("\tConsidering node: {node}({node:#b}), {distance} steps");

            // check if we've reached the solution
            if node == 0 {
                #[cfg(feature = "verbose")]
                eprintln!("Found a path - {path:?} - after considering {} nodes", visited_nodes.iter().count());
                return Some(path)
            }

            // try pressing each button
            for &button in &self.buttons {
                // find where we'd be if we pressed this button
                let next_node = node ^ button;

                // check that we have a valid state

                // #[cfg(feature = "verbose")]
                // eprintln!("\t\tIf we press {button}({button:#b}) we reach {next_node}({next_node:#b}) in {} steps", distance + 1);


                // if we're going back to a state we've already visited, ignore this button
                if visited_nodes.contains(&next_node) {
                    // #[cfg(all(feature = "verbose", feature = "part1"))]
                    // eprintln!("\t\t\tWe've already considered {next_node}({next_node:#b})");
                    continue
                }
                
                // if we've already got a path to this node
                if let Some((index, (_, current_distance))) = next_nodes.iter_mut().enumerate().find(|(_, (n, _))| *n == next_node) {
                    // if the path is shorter, ignore the new one
                    if current_distance.len() <= path.len() + 1 {
                        // #[cfg(feature = "verbose")]
                        // eprintln!("\t\t\tWe already have a route to {next_node:?} in {current_distance} steps");
                        continue
                    }

                    // otherwise, discard the old one
                    next_nodes.remove(index);
                }

                let mut new_path = path.clone();
                new_path.push(button);

                // put lower weights at the end so they are popped sooner
                let index = next_nodes.partition_point(|(n, p)| (p.len() > new_path.len()) && (n.count_zeros() > next_node.count_zeros()));
                
                next_nodes.insert(index, (next_node, new_path));

                // #[cfg(feature = "verbose")]
                // eprintln!("\t\t\t{next_nodes:?}");
            }
            
            visited_nodes.insert(node);
        }
    }
    
    pub fn bifurcate(&self) -> Option<usize> {
        let mut joltages = self.joltage_requirements.clone();
        let mut count = 0usize;
        let mut mult = 1usize;

        while joltages.iter().any(|&n| n > 0) {
            #[cfg(feature = "verbose")]
            eprintln!("Count is {count}, multiplier is {mult}, target is {joltages:?}");
            
            // work out which counts we need to bring down
            let bits = joltages.iter()
                .enumerate()
                .filter(|&(_, n)| n % 2 != 0)
                .map(|(i, _)| 1u16 << i)
                .sum();

            // work out how many button presses that will take
            let button_presses_to_make_even = self.a_star(bits)?;
            count += mult * button_presses_to_make_even.iter().count();
        
            // move on to the next bits 
            joltages.iter_mut()
                .enumerate()
                .for_each(|(index, joltage)| {
                    *joltage -= button_presses_to_make_even.iter()
                        .filter(|&button| (1 << index) & button != 0)
                        .count();

                    *joltage >>= 1;
                });

            mult <<= 1;
        }

        Some(count)
    }
}

fn main() {
    let mut input = get_input().unwrap();
    
    let regex = regex::Regex::new(r"\[(?<lights>[\.\#]*)\] (?<buttons>(\(([0-9]+,?)+\) ?)+) \{(?<joltage_requirements>([0-9]+,?)+)\}").unwrap();
    let machines = input.iter_by_regex::<Machine>(&regex);

    let total_button_presses: usize = machines
        .flat_map(|machine| {
            #[cfg(feature = "part1")]
            {
                let result = machine.a_star(machine.lights_bitset);
                if result.is_none() { eprintln!("Failed to solve: {machine:?}"); }
                return Some(result?.iter().count());
            }

            #[cfg(feature = "part2")]
            #[allow(unreachable_code)]
            {
                let result = machine.bifurcate();
                if result.is_none() { eprintln!("Failed to solve: {machine:?}"); }
                return result;
            }
        })
        .sum();

    println!("{total_button_presses}");
}
