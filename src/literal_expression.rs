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

pub fn decimal_integer(input: &str) -> IResult<&str, &str> {
    recognize(
        many1(
            terminated(one_of("0123456789"), many0(char('_')))
        )
    )(input)
}

pub fn decimal_float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(
            tuple((
                char('.'),
                decimal_integer,
                opt(tuple((
                    one_of("eE"),
                    opt(one_of("+-")),
                    decimal_integer
                )))
            ))
        ),
        // Case two: 42e42 and 42.42e42
        recognize(
            tuple((
                decimal_integer,
                opt(preceded(
                    char('.'),
                    decimal_integer,
                )),
                one_of("eE"),
                opt(one_of("+-")),
                decimal_integer
            ))
        ),
        // Case three: 42. and 42.42
        recognize(
            tuple((
                decimal_integer,
                char('.'),
                opt(decimal_integer)
            ))
        )
    ))(input)
}

#[derive(Debug)]
pub enum LiteralExpression {
    Float(String)
}

impl Instruction for LiteralExpression {
    fn to_rust(&self) -> String {
        let mut output = String::new();

        match self {
            LiteralExpression::Float(value) => output.push_str(&value.to_string()),
        }

        output
    }
}

impl LiteralExpression {
    pub fn parse(input: &str) -> IResult<&str, Expression> {
        let (remainder, value) = decimal_float(input)?;
        Ok((
            remainder,
            Expression::Literal(Self::Float(value.to_string()))
        ))
    }
}
