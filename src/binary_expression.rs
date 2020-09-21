use nom::{
    IResult,
    character::complete::*,
    bytes::complete::*,
    combinator::*,
    sequence::*,
    multi::*,
    branch::*,
};

use crate::instruction::*;
use crate::expression::*;

#[derive(Debug)]
pub struct BinaryExpression {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Instruction for BinaryExpression {
    fn to_rust(&self) -> String {
        let mut output = String::new();

        //output.push('(');
        output.push_str(&self.left.to_rust());
        output.push(' ');
        output.push_str(&self.operator);
        output.push(' ');
        output.push_str(&self.right.to_rust());
        //output.push(')');

        output
    }
}

fn binary_operator(input: &str) -> IResult<&str, &str> {
    alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
    ))(input)
}

fn binary_term(input: &str) -> IResult<&str, (&str, &str)> {
    pair(
        delimited(
            space0,
            recognize(many1(none_of(" "))),
            space0,
        ),
        binary_operator
    )(input)
}

fn binary_expression(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (remainder, (left, right)) = tuple((
        recognize(many1(binary_term)),
        preceded(
            space0,
            recognize(terminated(not_line_ending, opt(line_ending)))
        )
    ))(input)?;

    let (_, operator) = many1(binary_term)(input)?;
    let operator = operator.last().unwrap().1;

    Ok((
        remainder,
        (&left[..left.len() - operator.len()], operator, right)
    ))
}

impl BinaryExpression {
    pub fn parse(input: &str) -> IResult<&str, Expression> {
        let (remainder, (left, operator, right)) = binary_expression(input)?;

        let (_, left) = Expression::parse(left)?;
        let (_, right) = Expression::parse(right)?;
        //println!("Left: {:?}", left);
        //println!("Right: {:?}", right);

        Ok((
            remainder,
            Expression::Binary(Self {
                operator: operator.to_string(),
                left: Box::new(left),
                right: Box::new(right),
            })
        ))
    }
}