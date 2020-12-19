use crate::day::Day;
extern crate nom;

use self::nom::branch::alt;
use self::nom::bytes::complete::tag;
use self::nom::character::complete::digit1;
use self::nom::IResult;

pub struct Day18;

// Part1:
// <expr> :: <term> | <term><op><expr>
// <term> :: <open-paren><expr><close-paren> | <digit>
// <digit> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
// <open-paren> ::= "("
// <close-paren> ::= ")"
// op ::= " + " | " * "

#[derive(Clone)]
enum Expr1 {
    Term(Term1),
    Plus(Term1, Box<Expr1>),
    Times(Term1, Box<Expr1>),
}

fn plus(y: u64) -> impl Fn(u64) -> u64 {
    move |x| x + y
}

fn mult(y: u64) -> impl Fn(u64) -> u64 {
    move |x| x * y
}

impl Expr1 {
    fn term(term: Term1) -> Expr1 {
        Expr1::Term(term)
    }

    fn plus(term: Term1, expr: Expr1) -> Expr1 {
        Expr1::Plus(term, Box::new(expr))
    }

    fn times(term: Term1, expr: Expr1) -> Expr1 {
        Expr1::Times(term, Box::new(expr))
    }

    fn evaluate<F>(&self, cont: F) -> u64
    where
        F: Fn(u64) -> u64,
    {
        match self {
            Expr1::Term(term) => cont(term.evaluate()),
            Expr1::Plus(term, expr) => expr.evaluate(plus(cont(term.evaluate()))),
            Expr1::Times(term, expr) => expr.evaluate(mult(cont(term.evaluate()))),
        }
    }
}

#[derive(Clone)]
enum Term1 {
    Paren(Box<Expr1>),
    Digit(u64),
}

impl Term1 {
    fn paren(expr: Expr1) -> Term1 {
        Term1::Paren(Box::new(expr))
    }

    fn digit(value: u64) -> Term1 {
        Term1::Digit(value)
    }

    fn evaluate(&self) -> u64 {
        match self {
            Term1::Paren(expr) => expr.evaluate(|x| x),
            Term1::Digit(value) => *value,
        }
    }
}

fn digit(input: &str) -> IResult<&str, u64> {
    let (input, digit) = digit1(input)?;

    Ok((input, digit.parse::<u64>().unwrap()))
}

fn term1_digit(input: &str) -> IResult<&str, Term1> {
    let (input, value) = digit(input)?;

    Ok((input, Term1::digit(value)))
}

fn term1_paren(input: &str) -> IResult<&str, Term1> {
    let (input, _) = tag("(")(input)?;
    let (input, expr) = expr1(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Term1::paren(expr)))
}

fn term1(input: &str) -> IResult<&str, Term1> {
    let (input, term) = alt((term1_paren, term1_digit))(input)?;

    Ok((input, term))
}

fn expr1_times(input: &str) -> IResult<&str, Expr1> {
    let (input, term) = term1(input)?;
    let (input, _) = tag(" * ")(input)?;
    let (input, expr) = expr1(input)?;

    Ok((input, Expr1::times(term, expr)))
}

fn expr1_plus(input: &str) -> IResult<&str, Expr1> {
    let (input, term) = term1(input)?;
    let (input, _) = tag(" + ")(input)?;
    let (input, expr) = expr1(input)?;

    Ok((input, Expr1::plus(term, expr)))
}

fn expr1_term(input: &str) -> IResult<&str, Expr1> {
    let (input, term) = term1(input)?;

    Ok((input, Expr1::term(term)))
}

fn expr1(input: &str) -> IResult<&str, Expr1> {
    let (input, expr) = alt((expr1_times, expr1_plus, expr1_term))(input)?;

    Ok((input, expr))
}

fn parse1(line: String) -> Expr1 {
    expr1(&line[..]).unwrap().1
}

// Part2:
// <expr> ::= <factor> | <factor><times><expr>
// <factor> ::= <term> | <term><plus><factor>
// <term> ::= <open-paren><expr><close-paren> | <digit>
// <digit> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
// <open-paren> ::= "("
// <close-paren> ::= ")"
// <plus> ::= " + "
// <times> ::= " * "

enum Expr2 {
    Factor(Factor2),
    Times(Factor2, Box<Expr2>),
}

impl Expr2 {
    fn factor(factor: Factor2) -> Expr2 {
        Expr2::Factor(factor)
    }

    fn times(factor: Factor2, expr: Expr2) -> Expr2 {
        Expr2::Times(factor, Box::new(expr))
    }

    fn evaluate(&self) -> u64 {
        match self {
            Expr2::Factor(factor) => factor.evaluate(),
            Expr2::Times(factor, expr) => factor.evaluate() * expr.evaluate(),
        }
    }
}

enum Factor2 {
    Term(Term2),
    Plus(Term2, Box<Factor2>),
}

impl Factor2 {
    fn term(term: Term2) -> Factor2 {
        Factor2::Term(term)
    }

    fn plus(term: Term2, factor: Factor2) -> Factor2 {
        Factor2::Plus(term, Box::new(factor))
    }

    fn evaluate(&self) -> u64 {
        match self {
            Factor2::Term(term) => term.evaluate(),
            Factor2::Plus(term, factor) => term.evaluate() + factor.evaluate(),
        }
    }
}

enum Term2 {
    Paren(Box<Expr2>),
    Digit(u64),
}

impl Term2 {
    fn paren(expr: Expr2) -> Term2 {
        Term2::Paren(Box::new(expr))
    }

    fn digit(value: u64) -> Term2 {
        Term2::Digit(value)
    }

    fn evaluate(&self) -> u64 {
        match self {
            Term2::Paren(expr) => expr.evaluate(),
            Term2::Digit(value) => *value,
        }
    }
}

fn term2_paren(input: &str) -> IResult<&str, Term2> {
    let (input, _) = tag("(")(input)?;
    let (input, expr) = expr2(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Term2::paren(expr)))
}

fn term2_digit(input: &str) -> IResult<&str, Term2> {
    let (input, value) = digit(input)?;

    Ok((input, Term2::digit(value)))
}

fn term2(input: &str) -> IResult<&str, Term2> {
    let (input, term) = alt((term2_paren, term2_digit))(input)?;

    Ok((input, term))
}

fn factor2_term(input: &str) -> IResult<&str, Factor2> {
    let (input, term) = term2(input)?;

    Ok((input, Factor2::term(term)))
}

fn factor2_plus(input: &str) -> IResult<&str, Factor2> {
    let (input, term) = term2(input)?;
    let (input, _) = tag(" + ")(input)?;
    let (input, factor) = factor2(input)?;

    Ok((input, Factor2::plus(term, factor)))
}

fn factor2(input: &str) -> IResult<&str, Factor2> {
    let (input, factor) = alt((factor2_plus, factor2_term))(input)?;

    Ok((input, factor))
}

fn expr2_factor(input: &str) -> IResult<&str, Expr2> {
    let (input, factor) = factor2(input)?;

    Ok((input, Expr2::factor(factor)))
}

fn expr2_times(input: &str) -> IResult<&str, Expr2> {
    let (input, factor) = factor2(input)?;
    let (input, _) = tag(" * ")(input)?;
    let (input, expr) = expr2(input)?;

    Ok((input, Expr2::times(factor, expr)))
}

fn expr2(input: &str) -> IResult<&str, Expr2> {
    let (input, expr) = alt((expr2_times, expr2_factor))(input)?;

    Ok((input, expr))
}

fn parse2(line: String) -> Expr2 {
    expr2(&line[..]).unwrap().1
}

impl Day for Day18 {
    type T1 = u64;
    fn part1<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        input.map(|line| parse1(line).evaluate(|x| x)).sum()
    }

    type T2 = u64;
    fn part2<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        input.map(|line| parse2(line).evaluate()).sum()
    }
}
