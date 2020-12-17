use crate::day::Day;

use std::collections::HashMap;
use std::collections::HashSet;
struct InclusiveConstraint {
    lower: u64,
    upper: u64,
}

impl InclusiveConstraint {
    fn new(lower: u64, upper: u64) -> InclusiveConstraint {
        InclusiveConstraint { lower, upper }
    }

    fn contains(&self, value: u64) -> bool {
        return self.lower <= value && value <= self.upper;
    }
}

struct Constraints {
    constraints: Vec<InclusiveConstraint>,
}

struct Ticket {
    values: Vec<u64>,
}

impl Constraints {
    fn new(constraints: Vec<InclusiveConstraint>) -> Constraints {
        Constraints { constraints }
    }

    fn contains(&self, value: u64) -> bool {
        self.constraints
            .iter()
            .any(|constraint| constraint.contains(value))
    }
}

impl Ticket {
    fn new(values: Vec<u64>) -> Ticket {
        Ticket { values }
    }
}

struct Input {
    constraints: HashMap<String, Constraints>,
    my_ticket: Ticket,
    observed_tickets: Vec<Ticket>,
}

pub struct Day16;

fn parse_constraint(line: String) -> (String, Constraints) {
    let mut parts = line.split(": ");
    let name = parts.next().unwrap();
    let constraint_parts = parts.next().unwrap().split(" or ");

    (
        name.to_string(),
        Constraints::new(
            constraint_parts
                .map(|part| {
                    let mut sub_parts = part.split("-");
                    InclusiveConstraint::new(
                        sub_parts.next().unwrap().parse::<u64>().unwrap(),
                        sub_parts.next().unwrap().parse::<u64>().unwrap(),
                    )
                })
                .collect(),
        ),
    )
}

fn parse_ticket(line: String) -> Ticket {
    Ticket::new(line.split(",").map(|s| s.parse::<u64>().unwrap()).collect())
}

fn parse<I>(mut input: I) -> Input
where
    I: Iterator<Item = String>,
{
    let mut constraints = HashMap::<String, Constraints>::new();
    while let Some(line) = input.next() {
        if line.len() == 0 {
            break;
        }

        let (name, constraint) = parse_constraint(line);
        constraints.insert(name, constraint);
    }

    let mut token = input.next().unwrap();
    if token != "your ticket:" {
        panic!("Mising your ticket header! \"{}\"", token);
    }

    let my_ticket = parse_ticket(input.next().unwrap());

    token = input.next().unwrap();
    if token != "" {
        panic!("Missing empty line! \"{}\"", token);
    }
    token = input.next().unwrap();
    if token != "nearby tickets:" {
        panic!("Missing nearby ticket header! \"{}\"", token);
    }

    let observed_tickets: Vec<Ticket> = input.map(parse_ticket).collect();

    Input {
        constraints,
        my_ticket,
        observed_tickets,
    }
}

impl Day for Day16 {
    type T1 = u64;
    fn part1<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        let observations = parse(input);

        observations
            .observed_tickets
            .iter()
            .flat_map(|ticket| {
                ticket.values.iter().filter(|value| {
                    !observations
                        .constraints
                        .values()
                        .any(|constraint| constraint.contains(**value))
                })
            })
            .sum()
    }

    type T2 = u64;
    fn part2<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        let observations = parse(input);
        // let observations = test_input();
        let num_fields = observations.constraints.len();
        let fields: HashSet<String> = observations.constraints.keys().cloned().collect();
        let mut possibilities = vec![fields; num_fields];

        for ticket in &observations.observed_tickets {
            // Invalid ticket
            if !ticket.values.iter().all(|value| {
                observations
                    .constraints
                    .values()
                    .any(|constraint| constraint.contains(value.clone()))
            }) {
                continue;
            }

            possibilities = ticket
                .values
                .iter()
                .zip(possibilities)
                .map(|(value, possibility)| {
                    possibility
                        .iter()
                        .filter(|field| {
                            let constraint = observations.constraints.get(*field).unwrap();
                            constraint.contains(*value)
                        })
                        .cloned()
                        .collect()
                })
                .collect();
        }

        let mut found = HashSet::<String>::new();
        let mut result = 1;

        while found.len() < possibilities.len() {
            for i in 0..possibilities.len() {
                let possibility = &possibilities[i];
                let remainder: Vec<String> = possibility.difference(&found).cloned().collect();
                if remainder.len() != 1 {
                    continue;
                }
                let found_field = &remainder[0];
                found.insert(found_field.clone());
                if found_field.starts_with("departure") {
                    result *= observations.my_ticket.values[i];
                }
            }
        }

        result
    }
}
