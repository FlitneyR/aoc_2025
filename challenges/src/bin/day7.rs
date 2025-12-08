use std::collections::BTreeMap;

const BEAM_START: char = 'S';
const EMPTY_CELL: char = '.';
const BEAM_SPLITTER: char = '^';

fn main() {
    let mut input = aoc_2025_common::get_input().unwrap();

    let start_index = input.lines().next().unwrap()
        .char_indices()
        .find(|&(_index, char)| char == BEAM_START)
        .map(|(index, _char)| index)
        .unwrap();

    let mut beams = BTreeMap::new();
    beams.insert(start_index, 1);

    #[allow(unused)] // used in part1, but not part2
    let mut num_splits = 0;

    for line in input.lines() {
        let mut new_beams = BTreeMap::new();

        for (beam_index, beam_count) in beams {
            // eprintln!("{beam_count} beams at index {beam_index}");
            match line.as_str().chars().nth(beam_index) {
                Some(BEAM_SPLITTER) => {
                    *new_beams.entry(beam_index - 1).or_default() += beam_count;
                    *new_beams.entry(beam_index + 1).or_default() += beam_count;
                    num_splits += 1;
                },
                Some(EMPTY_CELL) => { *new_beams.entry(beam_index).or_default() += beam_count },
                Some(c) => panic!("Unexpected character: {c}"),
                None => panic!("Index {beam_index} is out of range"),
            }
        }

        #[cfg(feature = "verbose")]
        {
            for (index, char) in line.char_indices() {
                let num_beams = new_beams.get(&index);
                match (char, num_beams) {
                    ('.', Some(num_beams)) => print!("{num_beams}"),
                    (c, None) => print!("{c}"),
                    p => panic!("Unexpected: {p:?}"),
                }
            }
            println!();
        }

        beams = new_beams;
    }

    #[cfg(feature = "part1")]
    { println!("{num_splits}"); }

    #[cfg(feature = "part2")]
    {
        let total_particles: usize = beams.values().sum();
        println!("{total_particles}");
    }
}