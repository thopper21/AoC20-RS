use crate::day::Day;

use std::collections::HashMap;
use std::collections::HashSet;

pub struct Day7;

// <rule> := <plural_bag> <space> "contain" <space> <bag_list> "."
// <plural_bag> := <bag> <space> "bags"
// <bag_list> := <bag_quantities> | "no other bags"
// <bag_quantities> := <bag_quantity> "," <space> <bag_quantities> | <bag_quantity>
// <bag_quantity> := "1" <space> <single_bag> | <multiple> <space> <plural_bag>
// <descriptor> := <word>
// <color> := <word>
// <single_bag> := <bag> <space> "bag"
// <bag> := <descriptor> <space> <color>
// <multiple> := any number greater than 1
// <word> := any lower case word (a-z)
// <space> := " "

#[derive(PartialEq, Eq, Hash, Clone)]
struct Bag {
    descriptor: String,
    color: String,
}

fn bag(descriptor: &str, color: &str) -> Bag {
    Bag {
        descriptor: descriptor.to_string(),
        color: color.to_string(),
    }
}

struct BagQuantity {
    bag: Bag,
    count: usize,
}

struct Rule {
    container: Bag,
    contains: Vec<BagQuantity>,
}

fn parse_rule(line: String) -> Rule {
    let skip_words: HashSet<&str> = [
        "bags", "bag", "bags.", "bag.", "bags,", "bag,", "contain", "no", "other",
    ]
    .iter()
    .cloned()
    .collect();
    let mut bag_words = line
        .split_whitespace()
        .filter(|word| !skip_words.contains(word));
    let container_descriptor = bag_words.next().unwrap();
    let container_color = bag_words.next().unwrap();

    let mut contained = Vec::<BagQuantity>::new();
    while let Some(contained_count) = bag_words.next() {
        let contained_descriptor = bag_words.next().unwrap();
        let contained_color = bag_words.next().unwrap();

        contained.push(BagQuantity {
            bag: bag(contained_descriptor, contained_color),
            count: contained_count.parse::<usize>().unwrap(),
        })
    }

    Rule {
        container: bag(container_descriptor, container_color),
        contains: contained,
    }
}

fn bag_contained(graph: &HashMap<Bag, Vec<Bag>>, bag: &Bag) -> HashSet<Bag> {
    match graph.get(&bag) {
        Some(outer_bags) => {
            let mut result = HashSet::<Bag>::new();

            for outer_bag in outer_bags {
                result.insert(outer_bag.clone());
                result = result
                    .union(&bag_contained(&graph, &outer_bag))
                    .cloned()
                    .collect();
            }

            result
        }
        None => HashSet::<Bag>::new(),
    }
}

fn bag_count(graph: &HashMap<Bag, Vec<BagQuantity>>, bag: &Bag) -> usize {
    let mut sum = 0;

    for quantity in graph.get(bag).unwrap() {
        sum += quantity.count * (1 + bag_count(&graph, &quantity.bag));
    }

    sum
}

impl Day for Day7 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        let rules = input.map(parse_rule);
        let needle = bag("shiny", "gold");
        let mut graph = HashMap::<Bag, Vec<Bag>>::new();

        for rule in rules {
            for quantity in rule.contains {
                if let Some(contains) = graph.get_mut(&quantity.bag) {
                    contains.push(rule.container.clone());
                } else {
                    graph.insert(quantity.bag, vec![rule.container.clone()]);
                }
            }
        }

        bag_contained(&graph, &needle).len()
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        let rules = input.map(parse_rule);
        let needle = bag("shiny", "gold");
        let mut graph = HashMap::<Bag, Vec<BagQuantity>>::new();

        for rule in rules {
            graph.insert(rule.container, rule.contains);
        }

        bag_count(&graph, &needle)
    }
}
