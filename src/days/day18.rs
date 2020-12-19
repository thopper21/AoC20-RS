use crate::day::Day;
extern crate nom;

use self::nom::branch::alt;
use self::nom::bytes::complete::tag;
use self::nom::character::complete::char;
use self::nom::character::complete::digit1;
use self::nom::IResult;

pub struct Day18;

// Part1:
// expr ::= <term> | <term><space><op><space><expr>
// term ::= <paren> | <digit>
// paren ::= <open-paren><expr><close-paren>
// digit ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
// open-paren ::= "("
// close-paren ::= ")"
// op ::= "+" | "*"
// space ::= " "

#[derive(Clone, Copy)]
enum Operation {
    Plus,
    Times,
}

#[derive(Clone)]
enum Term {
    Paren(Box<Expression>),
    Digit(u64),
}

impl Term {
    fn paren(expression: Expression) -> Term {
        Term::Paren(Box::new(expression))
    }

    fn digit(value: u64) -> Term {
        Term::Digit(value)
    }

    fn evaluate(&self) -> u64 {
        match self {
            Term::Paren(expression) => expression.evaluate(),
            Term::Digit(value) => *value,
        }
    }
}

#[derive(Clone)]
enum Expression {
    End(Term),
    Operation(Term, Operation, Box<Expression>),
}

impl Expression {
    fn end(term: Term) -> Expression {
        Expression::End(term)
    }

    fn operation(left: Term, op: Operation, right: Expression) -> Expression {
        Expression::Operation(left, op, Box::new(right))
    }

    fn evaluate(&self) -> u64 {
        let eval = |left: &Term, oper: &Operation, right: &Term| match *oper {
            Operation::Plus => left.evaluate() + right.evaluate(),
            Operation::Times => left.evaluate() * right.evaluate(),
        };

        let tail = |left: &Term, oper: &Operation, expr: &Expression| -> u64 {
            match expr {
                Expression::End(right) => eval(left, oper, right),
                Expression::Operation(right, op_next, next) => {
                    Expression::Operation(Term::digit(eval(left, oper, right)), op_next.clone(), next.clone())
                        .evaluate()
                }
            }
        };

        match self {
            Expression::End(term) => term.evaluate(),
            Expression::Operation(left, op, expr) => tail(left, op, &*expr),
        }
    }
}

fn op(line: &str) -> IResult<&str, Operation> {
    let (line, _) = tag(" ")(line)?;
    let (line, op) = alt((char('*'), char('+')))(line)?;
    let (line, _) = tag(" ")(line)?;

    Ok((
        line,
        match op {
            '+' => Operation::Plus,
            '*' => Operation::Times,
            _ => unreachable!(),
        },
    ))
}

fn digit(line: &str) -> IResult<&str, Term> {
    let (line, digit) = digit1(line)?;

    Ok((line, Term::digit(digit.parse::<u64>().unwrap())))
}

fn paren(line: &str) -> IResult<&str, Term> {
    let (line, _) = tag("(")(line)?;
    let (line, expr) = expression(line)?;
    let (line, _) = tag(")")(line)?;

    Ok((line, Term::paren(expr)))
}

fn term(line: &str) -> IResult<&str, Term> {
    alt((paren, digit))(line)
}

fn operation(line: &str) -> IResult<&str, Expression> {
    let (line, left) = term(line)?;
    let (line, op) = op(line)?;
    let (line, right) = expression(line)?;

    Ok((line, Expression::operation(left, op, right)))
}

fn end(line: &str) -> IResult<&str, Expression> {
    let (line, term) = term(line)?;

    Ok((line, Expression::end(term)))
}

fn expression(line: &str) -> IResult<&str, Expression> {
    alt((operation, end))(line)
}

fn parse1(line: String) -> Expression {
    expression(&line[..]).unwrap().1
}

impl Day for Day18 {
    type T1 = u64;
    fn part1<I>(input: I) -> u64
    where
        I: Iterator<Item = String>,
    {
        input.map(|line| {
            parse1(line).evaluate()
        }).sum()
    }

    type T2 = usize;
    fn part2<I>(input: I) -> usize
    where
        I: Iterator<Item = String>,
    {
        // 1 + (2 * 3) + (4 * (5 + 6))
        // expr(sum(sum(1, prod(2, 3)), prod(4, sum(5, 6))))
        input.count()
    }
}
