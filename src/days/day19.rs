use crate::day::Day;
extern crate nom;

use std::collections::HashMap;

use self::nom::branch::alt;
use self::nom::bytes::complete::tag;
use self::nom::character::complete::anychar;
use self::nom::character::complete::digit1;
use self::nom::IResult;

pub struct Day19;

struct Input {
    rules: RuleSet,
    messages: Vec<String>,
}

impl Input {
    fn count_valid(&self) -> usize {
        self.messages
            .iter()
            .filter(|message| match self.rules.eval(message, 0) {
                Some(remainder) => remainder.is_empty(),
                None => false,
            })
            .count()
    }
}

struct RuleSet {
    rules: HashMap<usize, Rule>,
}

impl RuleSet {
    fn eval<'a>(&self, input: &'a str, i: usize) -> Option<&'a str> {
        self.eval_rule(input, &self.rules.get(&i).unwrap())
    }

    fn eval_rule<'a>(&self, input: &'a str, rule: &Rule) -> Option<&'a str> {
        match rule {
            Rule::Or(left, right) => self
                .eval_rule(input, &*left)
                .or_else(|| self.eval_rule(input, &*right)),
            Rule::Seq(left, right) => self
                .eval_rule(input, &*left)
                .and_then(|input| self.eval_rule(input, &*right)),
            Rule::Tag(val) => {
                input
                    .chars()
                    .next()
                    .and_then(|c| if c == *val { Some(&input[1..]) } else { None })
            }
            Rule::Hole(i) => self.eval(input, *i),
        }
    }
}

enum Rule {
    Or(Box<Rule>, Box<Rule>),
    Seq(Box<Rule>, Box<Rule>),
    Tag(char),
    Hole(usize),
}

impl Rule {
    fn or(left: Rule, right: Rule) -> Rule {
        Rule::Or(Box::new(left), Box::new(right))
    }

    fn seq(left: Rule, right: Rule) -> Rule {
        Rule::Seq(Box::new(left), Box::new(right))
    }

    fn tag(val: char) -> Rule {
        Rule::Tag(val)
    }

    fn hole(i: usize) -> Rule {
        Rule::Hole(i)
    }
}

fn rule_seq(input: &str) -> IResult<&str, Rule> {
    let (input, left) = rule_hole(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, right) = alt((rule_seq, rule_hole))(input)?;

    Ok((input, Rule::seq(left, right)))
}

fn rule_hole(input: &str) -> IResult<&str, Rule> {
    let (input, left) = digit1(input)?;

    Ok((input, Rule::hole(left.parse::<usize>().unwrap())))
}

fn rule_or(input: &str) -> IResult<&str, Rule> {
    let (input, left) = alt((rule_seq, rule_hole))(input)?;
    let (input, _) = tag(" | ")(input)?;
    let (input, right) = rule(input)?;

    Ok((input, Rule::or(left, right)))
}

fn rule_tag(input: &str) -> IResult<&str, Rule> {
    let (input, _) = tag("\"")(input)?;
    let (input, c) = anychar(input)?;
    let (input, _) = tag("\"")(input)?;

    Ok((input, Rule::tag(c)))
}

fn rule_dep(input: &str) -> IResult<&str, Rule> {
    let (input, rule) = alt((rule_or, rule_seq, rule_hole))(input)?;

    Ok((input, rule))
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, rule) = alt((rule_tag, rule_dep))(input)?;

    Ok((input, rule))
}

fn rule_line(input: &str) -> IResult<&str, (usize, Rule)> {
    let (input, index) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rule) = rule(input)?;

    Ok((input, (index.parse::<usize>().unwrap(), rule)))
}

fn parse_rule_line(line: String) -> (usize, Rule) {
    rule_line(&line[..]).unwrap().1
}

fn parse<I>(mut input: I) -> Input
where
    I: Iterator<Item = String>,
{
    let mut rules = HashMap::<usize, Rule>::new();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }

        let (i, rule) = parse_rule_line(line);
        rules.insert(i, rule);
    }

    let mut messages = Vec::<String>::new();
    while let Some(line) = input.next() {
        messages.push(line)
    }

    Input {
        rules: RuleSet { rules },
        messages,
    }
}

impl Day for Day19 {
    type T1 = usize;
    fn part1<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        parse(input).count_valid()
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        let mut x = parse(input);
        x.rules.rules.insert(
            8,
            Rule::or(Rule::hole(42), Rule::seq(Rule::hole(42), Rule::hole(8))),
        );
        x.rules.rules.insert(
            11,
            Rule::or(
                Rule::seq(Rule::hole(42), Rule::hole(31)),
                Rule::seq(Rule::hole(42), Rule::seq(Rule::hole(11), Rule::hole(31))),
            ),
        );

        // match x.rules.eval("babbbbaabbbbbabbbbbbaabaaabaaa", 0) {
        //     Some(remainder) => {
        //         println!("Success: {}", remainder)
        //     }
        //     None => {
        //         println!("Fail");
        //     }
        // }

        // 42

        x.count_valid()
    }
}
