use aoc_2025_common::*;
use aoc_2025_proc_macros::*;
use std::cell::Cell;

#[derive(Clone, Debug, PartialEq, FromRegexCaptures)]
struct TileCoords {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct AABB {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    area_cache: Cell<Option<usize>>,
}

impl AABB {
    pub fn from_tiles(a: &TileCoords, b: &TileCoords) -> Self {
        Self {
            min_x: a.x.min(b.x),
            min_y: a.y.min(b.y),
            max_x: a.x.max(b.x),
            max_y: a.y.max(b.y),
            area_cache: Cell::new(None),
        }
    }

    pub fn area(&self) -> usize {
        if self.area_cache.get().is_none() { self.area_cache.set(Some((self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1))) }
        self.area_cache.get().unwrap()
    }

    #[cfg(feature = "part2")]
    pub fn intersects(lhs: &Self, rhs: &Self) -> bool {
        !( lhs.max_x <= rhs.min_x
        || rhs.max_x <= lhs.min_x
        || lhs.max_y <= rhs.min_y
        || rhs.max_y <= lhs.min_y )
    }
}

fn main() {
    let red_tile_coords: Box<[_]> = get_input().unwrap()
        .iter_by_regex::<TileCoords>(&regex::Regex::new("(?<x>[0-9]+),(?<y>[0-9]+)").unwrap())
        .collect();

    #[cfg(feature = "part2")]
    let tile_strips: Box<[_]> = red_tile_coords.iter()
        .wrap_around(1)
        .windows(2)
        .map(|tiles| (tiles[0], tiles[1]))
        .collect();

    let mut aabbs: Box<[_]> = red_tile_coords.iter()
        .enumerate()
        .flat_map(|(index, lhs)| red_tile_coords.iter().skip(index).map(move |rhs| (lhs, rhs)))
        .map(|(lhs, rhs)| AABB::from_tiles(lhs, rhs))
        .collect();
    
    aabbs.sort_by_key(|aabb| aabb.area());
    aabbs.reverse();

    #[cfg(feature = "part1")]
    #[allow(unused)]
    let maximum_area: usize = aabbs[0].area();

    #[cfg(feature = "part2")]
    let maximum_area = aabbs.iter()
        .filter(|aabb|
            !tile_strips.iter()
                .any(|(from, to)| AABB::intersects(aabb, &AABB::from_tiles(from, to)))
        )
        .next()
        .unwrap()
        .area();
    
    println!("{maximum_area}");
}
