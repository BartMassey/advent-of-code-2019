// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 14.  
//! Bart Massey 2019

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Component {
    quantity: u64,
    name: String,
}

impl Component {
    fn new(desc: &str) -> Self {
        let component: Vec<&str> =
            desc.split(' ').collect();
        assert_eq!(2, component.len());
        let quantity = component[0].parse().expect("bad component count");
        assert!(quantity > 0);
        let name = component[1].to_owned();
        Self { quantity, name }
    }
}

#[derive(Debug, Clone)]
struct Reaction {
    lhs: Vec<Component>,
    rhs: Component,
}

impl Reaction {
    fn new<S>(desc: S) -> Self
        where S: AsRef<str>
    {
        let sides: Vec<&str> =
            desc.as_ref().split(" => ").collect();
        assert_eq!(2, sides.len());
        let rhs = Component::new(sides[1]);
        let lhs: Vec<Component> = sides[0]
            .split(", ")
            .map(|c| Component::new(c))
            .collect();
        Self { lhs, rhs }
    }
}

type ReactionMap = HashMap<String, Reaction>;

fn make_reaction_map(reactions: &[Reaction]) -> ReactionMap {
    reactions
        .iter()
        .map(|r| (r.rhs.name.clone(), r.clone()))
        .collect()
}

struct Product(HashMap<String, u64>);

impl Product {

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add(&mut self, name: &str, quantity: u64) {
        let entry = self.0.entry(name).or_insert(0);
        *entry += quantity;
    }

    fn sub(&mut self, name: &str, quantity: u64) {
        let entry = self.0.entry(name).or_insert(0);
        assert!(*entry >= quantity);
        *entry -= quantity;
    }

    fn get(&mut self, name: &str) -> u64 {
        self.get(name).or(0)
    }
}

fn min_ore(
    target: &str,
    quantity: u64,
    reactions: &ReactionMap,
    ) -> (u64, Product)
{
    let mut product = Product::new();
    if quantity == 0 {
        return (0, product);
    }
    if target == "ORE" {
        product.add("ORE", quantity);
        return (quantity, product);
    }

    let parent = reactions
        .get(target)
        .expect("no way to make component");

    // Amount of product per reaction.
    let nr = parent.rhs.quantity;
    // Number of times to run the reaction.
    let r = (quantity + nr - 1) / nr;

    // Find the net products for this reaction.
    for c in parent.lhs {
        // Amount of component needed.
        let n = r * c.quantity;
        let (q, p) = min_ore(&c.name, n, reactions);
        product.add_map(p);
    });
}

fn parse_desc<S>(desc: impl Iterator<Item=S>) -> Vec<Reaction>
    where S: AsRef<str>
{
    desc.map(Reaction::new).collect()
}

fn find_min_ore<S>(lines: impl Iterator<Item=S>) -> u64
    where S: AsRef<str>
{
    let reactions = parse_desc(lines);
    let map = make_reaction_map(&reactions);
    min_ore("FUEL", 1, &map)
}

#[test]
fn test_find_min_ore() {
    let test1 = &[
        "10 ORE => 10 A",
        "1 ORE => 1 B",
        "7 A, 1 B => 1 C",
        "7 A, 1 C => 1 D",
        "7 A, 1 D => 1 E",
        "7 A, 1 E => 1 FUEL",
    ];
    assert_eq!(31, find_min_ore(test1.iter()));
}

pub fn main() {
    let part = aoc::get_part();
    assert!(part == aoc::Part1);
    let lines = aoc::input_lines();
    println!("{}", find_min_ore(lines));
}
