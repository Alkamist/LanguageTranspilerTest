use nom::{
    IResult,
    character::complete::{
        char,
        one_of,
    },
    combinator::{
        recognize,
        opt,
        map,
    },
    sequence::{
        tuple,
        terminated,
        preceded,
    },
    multi::{
        many0,
        many1,
    },
    branch::alt,
};

use crate::instruction::*;

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

//pub enum Literal {
//    Float(f64)
//}
//
//impl Instruction for Literal {
//    fn to_rust(&self) -> String {
//        match self {
//            Literal::Float(value) => value.to_string(),
//        }
//    }
//}
//
//impl Literal {
//    pub fn parse(input: &str) -> IResult<&str, Self> {
//        map(
//            decimal_float,
//            |value| Literal::Float(value.parse::<f64>().unwrap())
//        )(input)
//    }
//}
