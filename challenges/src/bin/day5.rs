use aoc_2025_common::FromRegexCaptures;
use aoc_2025_proc_macros::FromRegexCaptures;

#[derive(Copy, Clone, Debug, FromRegexCaptures)]
struct IDRange {
    start: usize,
    end: usize,
}

impl IDRange {
    #[allow(unused)] // this is used in part1, but not part2
    pub fn contains(&self, id: usize) -> bool { self.try_to_natve().map(|r| r.contains(&id)).unwrap_or(false) }

    #[allow(unused)] // this is used in part2, but not part1
    pub fn count(&self) -> usize { self.try_to_natve().map(|r| r.count()).unwrap_or(0) }

    #[allow(unused)] // this is used in part2, but not part1
    pub fn is_valid(&self) -> bool { self.start <= self.end }

    pub fn try_to_natve(&self) -> Option<std::ops::RangeInclusive<usize>> {
        if !self.is_valid() { None }
        else { Some(self.start..=self.end) }
    }
}

fn main() {
    assert_eq!(IDRange{ start: 3, end: 15 }.count(), 13);

    let mut input = aoc_2025_common::get_input().unwrap();
    let range_regex = regex::Regex::new("(?<start>[0-9]+)-(?<end>[0-9]+)").unwrap();
    
    // can be const for part1, but needs to be mutable for part2
    #[allow(unused_mut)]
    let mut ranges: Box<[IDRange]> = input.lines()
        .map_while(|line| {
            let captures = range_regex.captures(&line)?;
            IDRange::from_regex_captures(&captures).ok()
        }).collect();

    #[cfg(feature = "part1")]
    {
        let fresh_count = input.lines()
            .map_while(|line| line.parse().ok())
            .filter(|id: &usize| ranges.iter().any(|range| range.contains(*id)))
            .count();
        
        eprintln!("{fresh_count}");
    }

    #[cfg(feature = "part2")]
    {
        ranges.sort_by_key(|range| range.start);

        // make sure each range starts after the previous range ends
        for i in 0..ranges.len() - 1 {
            ranges[i + 1].start = ranges[i + 1].start.max(ranges[i].end + 1);
            ranges[i + 1].end = ranges[i + 1].end.max(ranges[i + 1].start - 1);
        }

        let sum: usize = ranges.iter()
            .map(|r| r.count())
            .sum();
        
        println!("{sum}");
    }
}
