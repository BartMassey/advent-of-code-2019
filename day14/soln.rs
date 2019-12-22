// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 14.  
//! Bart Massey 2019

use std::collections::HashMap;
use topological_sort::TopologicalSort;

/// A reaction component.
#[derive(Debug, Clone)]
struct Component {
    /// Fundamental quantity of component used in reaction.
    quantity: u64,
    /// Name of component.
    name: String,
}

impl Component {
    /// Parse a new component from a string in problem format.
    fn new(desc: &str) -> Self {
        let component: Vec<&str> = desc.split(' ').collect();
        assert_eq!(2, component.len());
        let quantity =
            component[0].parse().expect("bad component count");
        assert!(quantity > 0);
        let name = component[1].to_owned();
        Self { quantity, name }
    }
}

/// A reaction takes components and produces a single
/// component.
#[derive(Debug, Clone)]
struct Reaction {
    /// Reactants.
    lhs: Vec<Component>,
    /// Product.
    rhs: Component,
}

impl Reaction {
    /// Parse a reaction from a description string.
    fn new<S>(desc: S) -> Self
    where
        S: AsRef<str>,
    {
        let sides: Vec<&str> = desc.as_ref().split(" => ").collect();
        assert_eq!(2, sides.len());
        let rhs = Component::new(sides[1]);
        let lhs: Vec<Component> =
            sides[0].split(", ").map(|c| Component::new(c)).collect();
        Self { lhs, rhs }
    }
}

/// A "reaction map" maps product names to the reactions
/// that produce them.
type ReactionMap = HashMap<String, Reaction>;

/// Make a reaction map from a list of reactions.
fn make_reaction_map(reactions: &[Reaction]) -> ReactionMap {
    reactions
        .iter()
        .map(|r| (r.rhs.name.clone(), r.clone()))
        .collect()
}

/// Reactants need to be produced before product. If
/// we traverse the components in product-to-input order
/// we can compute what we need to.
type ReactionOrder = Vec<String>;

/// Some topological sort of the reaction graph is an order
/// to run reactions in.
fn reaction_order(reactions: &ReactionMap) -> ReactionOrder {
    let mut ts = TopologicalSort::<&str>::new();
    for r in reactions.values() {
        let cr = r.rhs.name.as_ref();
        for cl in &r.lhs {
            ts.add_dependency(cl.name.as_ref(), cr);
        }
    }
    let nts = ts.len();
    let ro: ReactionOrder = ts.map(str::to_owned).collect();
    assert_eq!("ORE", ro[0]);
    assert_eq!("FUEL", ro[nts - 1]);
    ro
}


/// A reaction schema contains the data necessary to run
/// reactions.
#[derive(Debug, Clone)]
struct ReactionSchema {
    map: ReactionMap,
    order: ReactionOrder,
}

impl ReactionSchema {

    /// Compile reactions into a schema.
    fn new(reactions: &[Reaction]) -> Self {
        let map = make_reaction_map(reactions);
        let order = reaction_order(&map);
        Self { map, order }
    }
}

/// A "products list" is a table showing the needed
/// amount of various components.
struct Products(HashMap<String, u64>);

impl Products {
    /// Make a new empty products list.
    fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add some product demand to the product list.
    fn add(&mut self, name: &str, quantity: u64) {
        let entry = self.0.entry(name.to_string()).or_insert(0);
        *entry += quantity;
    }

    /// Get the current demand for some product.
    fn get(&mut self, name: &str) -> u64 {
        *self.0.get(name).unwrap_or(&0)
    }
}

// Tests from problem description.
#[cfg(test)]
#[rustfmt::skip]
const TESTS: &[(&[&str], u64, Option<u64>)] = &[
    (
        &[
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ],
        31,
        None,
    ),
    (
        &[
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ],
        165,
        None,
    ),
    (
        &[
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ],
        13312,
        Some(82892753),
    ),
    (
        &[
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ],
        180697,
        Some(5586022),
    ),
    (
        &[
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ],
        2210736,
        Some(460664),
    ),
];

/// Compute the minimum amount of ore needed to produce the
/// given amount of fuel using the given reactions.
fn min_ore(reactions: &ReactionSchema, fuel: u64) -> u64 {
    let mut products = Products::new();

    // Traverse the products from output to input, recording
    // the new demand at each stage relative to the amount
    // of the output needed. There is no reaction that
    // produces ORE, so don't look for one.
    products.add("FUEL", fuel);
    for target in reactions.order.iter().skip(1).rev() {
        let eqn = reactions.map.get(target).expect("no reaction found");
        assert_eq!(&eqn.rhs.name, target);
        let need = products.get(target);
        let q = eqn.rhs.quantity;
        // Need to round up the number of reactions because
        // reactions produce multiple outputs.
        let nreactions = (need + q - 1) / q;
        for reagent in &eqn.lhs {
            products.add(&reagent.name, reagent.quantity * nreactions);
        }
    }

    // We have now produced everything we need but ORE. And
    // we know how much ORE we need.
    products.get("ORE")
}

/// Take an iterator over lines describing reactions and
/// produce a reaction schema.
fn parse_reactions<S>(lines: impl Iterator<Item = S>) -> ReactionSchema
where
    S: AsRef<str>,
{
    let reactions: Vec<Reaction> = lines.map(Reaction::new).collect();
    ReactionSchema::new(&reactions)
}

#[test]
fn test_min_ore() {
    for (test, q, _) in TESTS.iter() {
        let reactions = parse_reactions(test.iter());
        assert_eq!(*q, min_ore(&reactions, 1));
    }
}

/// Find the maximum fuel that can be produced from the
/// given ore capacity using the given
/// reactions. Fundamental strategy is binary search
/// (suggested by folks on the Internet).
fn max_fuel(reactions: &ReactionSchema, ore_cap: u64) -> u64 {
    // Double fuel demand until ore capacity is exceeded.
    let mut upper = 1;
    while min_ore(reactions, upper) <= ore_cap {
        upper *= 2;
    }

    // Binary search the region between the two limits
    // iteratively until converged on a result.
    let mut lower = upper / 2;
    while lower + 1 < upper {
        let mid = lower + (upper - lower + 1) / 2;
        let ore = min_ore(reactions, mid);
        if ore <= ore_cap {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    lower
}

/// Amount of fuel to produce as given by the problem — an
/// inconvenient constant.
const TRILLION: u64 = 1_000_000_000_000;

#[test]
fn test_max_fuel() {
    for (test, _, q) in TESTS.iter() {
        if let Some(q) = q {
            let reactions = parse_reactions(test.iter());
            assert_eq!(*q, max_fuel(&reactions, TRILLION));
        }
    }
}

pub fn main() {
    let lines = aoc::input_lines();
    let reactions = parse_reactions(lines);
    let part = aoc::get_part();
    match part {
        aoc::Part1 => println!("{}", min_ore(&reactions, 1)),
        aoc::Part2 => println!("{}", max_fuel(&reactions, TRILLION)),
    }
}
