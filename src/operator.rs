use nom::{
    Err,
    IResult,
    character::complete::{
        char,
        multispace0,
        anychar,
        one_of,
        line_ending,
        not_line_ending,
    },
    sequence::{
        preceded,
        terminated,
        tuple,
    },
    combinator::{
        recognize,
        not,
        opt,
    },
    multi::{
        separated_list,
        many0,
        many1,
    },
    bytes::complete::{
        tag,
        is_not,
        take_until,
    },
    branch::{
        alt,
    }
};

//use crate::instruction::*;
//use crate::literal::*;

//pub fn add_subtract(input: &str) -> IResult<&str, Vec<&str>> {
//    let (remainder, asdf) = separated_list(
//        terminated(
//            one_of("+-"),
//            multispace0,
//        ),
//        tuple((
//            opt(tag("-")),
//            is_not("+-"),
//        ))
//    )(input)?;
//}

pub trait Instruction {
    fn to_rust(&self) -> String;
}

pub type Operator = String;


pub struct UnaryExpr {
    pub operator: Operator,
    pub value: Box<Expr>,
}

impl Instruction for UnaryExpr {
    fn to_rust(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.operator);
        output.push_str(&self.value.to_rust());
        output
    }
}

impl UnaryExpr {
    pub fn parse(input: &str) -> IResult<&str, Expr> {
        let (remainder, (operator, value)) = tuple((
            tag("-"),
            LiteralExpr::parse,
        ))(input)?;

        Ok((
            remainder,
            Expr::Unary(Self {
                operator: operator.to_string(),
                value: Box::new(value)
            })
        ))
    }
}


pub struct BinaryExpr {
    pub operator: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl Instruction for BinaryExpr {
    fn to_rust(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.left.to_rust());
        output.push(' ');
        output.push_str(&self.operator);
        output.push(' ');
        output.push_str(&self.right.to_rust());
        output
    }
}

impl BinaryExpr {
    pub fn parse(input: &str) -> IResult<&str, Expr> {
        let (remainder, (left, operator, right)) = tuple((
            is_not("+-*/"),
            alt((
                tag("+"),
                tag("-"),
                tag("*"),
                tag("/"),
            )),
            recognize(tuple((
                not_line_ending,
                opt(line_ending)
            )))
        ))(input)?;

        let (_, left) = Expr::parse(left)?;
        let (_, right) = Expr::parse(right)?;

        Ok((
            remainder,
            Expr::Binary(Self {
                operator: operator.to_string(),
                left: Box::new(left),
                right: Box::new(right),
            })
        ))
    }
}


//pub struct GroupingExpr {
//    pub value: Box<Expr>,
//}
//
//impl Instruction for GroupingExpr {
//    fn to_rust(&self) -> String {
//        let mut output = String::new();
//        output.push('(');
//        output.push_str(&self.value.to_rust());
//        output.push(')');
//        output
//    }
//}


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

pub enum LiteralExpr {
    Float(String)
}

impl Instruction for LiteralExpr {
    fn to_rust(&self) -> String {
        let mut output = String::new();

        match self {
            LiteralExpr::Float(value) => output.push_str(&value.to_string()),
        }

        output
    }
}

impl LiteralExpr {
    pub fn parse(input: &str) -> IResult<&str, Expr> {
        let (remainder, value) = decimal_float(input)?;
        Ok((
            remainder,
            Expr::Literal(Self::Float(value.to_string()))
        ))
    }
}


pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    //Grouping(GroupingExpr),
    Literal(LiteralExpr),
}

impl Instruction for Expr {
    fn to_rust(&self) -> String {
        match self {
            Expr::Unary(value) => value.to_rust(),
            Expr::Binary(value) => value.to_rust(),
            //Expr::Grouping(value) => value.to_rust(),
            Expr::Literal(value) => value.to_rust(),
        }
    }
}

impl Expr {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            BinaryExpr::parse,
            preceded(multispace0, UnaryExpr::parse),
            preceded(multispace0, LiteralExpr::parse),
        ))(input)
    }
}
