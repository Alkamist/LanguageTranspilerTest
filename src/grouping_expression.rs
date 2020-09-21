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
pub struct GroupingExpression {
    pub value: Box<Expression>,
}

impl Instruction for GroupingExpression {
    fn to_rust(&self) -> String {
        let mut output = String::new();
        output.push('(');
        output.push_str(&self.value.to_rust());
        output.push(')');
        output
    }
}

impl GroupingExpression {
    pub fn parse(input: &str) -> IResult<&str, Expression> {
        let (remainder, value) = delimited(
            tag("("),
            is_not("()"),
            tag(")"),
        )(input)?;

        let (_, value) = Expression::parse(value)?;

        Ok((
            remainder,
            Expression::Grouping(Self {
                value: Box::new(value)
            })
        ))
    }
}
