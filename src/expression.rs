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
use crate::grouping_expression::*;
use crate::literal_expression::*;
use crate::binary_expression::*;

#[derive(Debug)]
pub enum Expression {
    //Unary(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
    Literal(LiteralExpression),
}

impl Instruction for Expression {
    fn to_rust(&self) -> String {
        match self {
            //Expression::Unary(value) => value.to_rust(),
            Expression::Binary(value) => value.to_rust(),
            Expression::Grouping(value) => value.to_rust(),
            Expression::Literal(value) => value.to_rust(),
        }
    }
}

impl Expression {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            preceded(multispace0, BinaryExpression::parse),
            preceded(multispace0, GroupingExpression::parse),
            preceded(multispace0, LiteralExpression::parse),
            //preceded(multispace0, UnaryExpr::parse),
        ))(input)
    }
}