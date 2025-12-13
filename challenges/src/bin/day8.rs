use aoc_2025_common::*;
use aoc_2025_proc_macros::*;
use std::{cell::Cell, num::NonZeroUsize};

#[allow(unused)] // used by part1, not by part2
use std::collections::BTreeMap;

#[derive(Clone, Debug, FromRegexCaptures)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone, Debug)]
struct JunctionBox {
    point: Point,
    id: usize,
    circuit: Cell<Option<NonZeroUsize>>,
}

impl Point {
    pub fn sqr_distance(lhs: &Self, rhs: &Self) -> usize {
        let dx = lhs.x.abs_diff(rhs.x);
        let dy = lhs.y.abs_diff(rhs.y);
        let dz = lhs.z.abs_diff(rhs.z);
        (dx * dx) + (dy * dy) + (dz * dz)
    }
}

impl JunctionBox {
    pub fn connect(lhs: &Self, rhs: &Self, all_junction_boxes: &[Self], next_circuit_id: &mut usize) {
        match (lhs.circuit.get(), rhs.circuit.get()) {
            (None, None) => { // make a new circuit
                // unwrapping and rewrapping to make sure we have a valid NonZeroUsize here
                let circuit = Some(NonZeroUsize::new(*next_circuit_id).unwrap());
                lhs.circuit.set(circuit);
                rhs.circuit.set(circuit);
                *next_circuit_id += 1;
            },
            (Some(lhs), Some(rhs)) => { // merge two circuits
                if lhs == rhs { return } // already the same circuit, nothing to do here
                
                // choose one
                let merged_circuit_id = Some(lhs.min(rhs));
                let circuit_id_to_merge = Some(lhs.max(rhs));
                
                // move everything on one circuit to the other
                for junction_box in all_junction_boxes.iter().filter(|jb| jb.circuit.get() == circuit_id_to_merge) {
                    junction_box.circuit.set(merged_circuit_id);
                }
            },
            // if one has a circuit and the other doesn't, move that one to the other's circuit
            (Some(lhs), None) => { rhs.circuit.set(Some(lhs)); },
            (None, Some(rhs)) => { lhs.circuit.set(Some(rhs)); }, 
        }
    }
}

fn main() {
    // parse the input
    let junction_boxes: Box<[_]> = get_input().unwrap()
        .iter_by_regex(&regex::Regex::new("(?<x>[0-9]+),(?<y>[0-9]+),(?<z>[0-9]+)").unwrap())
        .enumerate()
        .map(|(id, point)| JunctionBox { id, point, circuit: Cell::new(None) })
        .collect();

    // make unique pairs
    let mut pairs: Box<[_]> = junction_boxes.iter()
        .map(|lhs| junction_boxes.iter()
            .skip(lhs.id + 1) // only look at junction boxes _after_ lhs
            .map(move |rhs| (lhs, rhs)))
        .flatten()
        .collect();

    // sort the pairs by distance
    pairs.sort_by_cached_key(|(lhs, rhs)| Point::sqr_distance(&lhs.point, &rhs.point));
    
    let mut next_circuit_id = 1usize;

    #[cfg(feature = "part1")]
    {
        // join up neighbours into circuits
        pairs.iter()
            .take(Arguments::get_named("count").unwrap())
            .for_each(|(lhs, rhs)| JunctionBox::connect(lhs, rhs, &junction_boxes, &mut next_circuit_id));

        // count the junction boxes in each circuit
        let mut circuit_sizes = BTreeMap::<usize, usize>::new();
        junction_boxes.iter()
            .filter_map(|junction_box| junction_box.circuit.get())
            .for_each(|circuit| *circuit_sizes.entry(circuit.get()).or_default() += 1 );

        #[cfg(feature="verbose")]
        eprintln!("Circuit Sizes: {circuit_sizes:?}");

        // sort the counts
        let mut largest_circuit_sizes: Box<[_]> = circuit_sizes.into_values().collect();
        largest_circuit_sizes.sort();

        // find the product of the three largest
        let product: usize = largest_circuit_sizes.iter().rev().take(3).product();
        println!("{product}");
    }

    #[cfg(feature = "part2")]
    {
        use std::num::NonZero;

        let mut pairs_iter = pairs.iter();
        let mut solution = 0usize;

        // when two circuits are mereged, they merge onto the smaller circuit
        // so when everything is on the same circuit, they should all be on circuit 1
        while junction_boxes.iter().any(|junction_box| junction_box.circuit.get() != Some(unsafe { NonZero::new_unchecked(1) }))
        {
            let (lhs, rhs) = match pairs_iter.next() {
                None => { eprintln!("No solution exists"); return },
                Some(pair) => pair,
            };

            JunctionBox::connect(lhs, rhs, &junction_boxes, &mut next_circuit_id);

            solution = lhs.point.x * rhs.point.x;
        }

        println!("{solution}");
    }
}
