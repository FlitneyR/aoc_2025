use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('+') => Ok(Operation::Add),
            Some('*') => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = aoc_2025_common::get_input().unwrap()
        .collect_to_string();

    #[cfg(feature = "part1")]
    {
        // parse the input
        let numbers_regex = regex::Regex::new("([0-9]+) *").unwrap();
        let numbers: Box<[usize]> = numbers_regex.captures_iter(&input)
            .map(|capture| capture.get(1).map(|s| s.as_str().parse()))
            .flatten()
            .flatten()
            .collect();

        let operations_regex = regex::Regex::new("[+*]").unwrap();
        let operations: Box<[Operation]> = operations_regex.find_iter(&input)
            .flat_map(|str| str.as_str().parse())
            .collect();

        // get rows of numbers
        assert_eq!(numbers.len() % operations.len(), 0);
        let number_rows: Box<[&[usize]]> = numbers.chunks(operations.len()).collect();

        // transmute into columns of numbers
        let number_cols: Box<[Box<[usize]>]> = (0..operations.len())
            .map(|index| number_rows.iter().map(move |row| row[index]).collect())
            .collect();

        let total_problem_sum: usize = operations.iter().zip(number_cols.iter())
            .map(|(operation, numbers)| match operation {
                Operation::Add => numbers.iter().sum::<usize>(),
                Operation::Multiply => numbers.iter().product::<usize>(),
            })
            .sum();

        println!("{total_problem_sum}");
    }

    #[cfg(feature = "part2")]
    {
        let operations_regex = regex::Regex::new("[+*] *").unwrap();
        
        // get all the rows that don't contain operations, these are numbers
        let number_rows: Box<[&str]> = input.lines()
           .take_while(|line| !operations_regex.is_match(&line))
           .collect();

        // get the row that does contain operations
        let first_line_with_operations = input.lines().skip(number_rows.len()).next().unwrap();
        let operations = operations_regex.find_iter(first_line_with_operations);

        let sum: usize = operations
            .map(|m| {
                let operation: Operation = m.as_str().parse().unwrap();
                // for each column of the problem
                let numbers: Box<[usize]> = (m.start()..m.end())
                    // get each digit at this index from the number rows
                    .map(|index| number_rows.iter()
                        .flat_map(move |row| row.chars().nth(index))
                        .filter(|char| char.is_numeric())
                        .collect()
                    )
                    .filter(|str: &String| !str.is_empty())
                    // parse as a number
                    .map(|str| str.parse::<usize>().unwrap())
                    .collect();

                match operation {
                    Operation::Add => numbers.iter().sum::<usize>(),
                    Operation::Multiply => numbers.iter().product::<usize>(),
                }
            })
            .sum();

        println!("{sum}")
    }
}
