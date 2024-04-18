pub mod ast;
pub mod expand;
pub mod latex;
pub mod lexer;
pub mod simplify;
pub mod operands;
pub mod factorization;

use rust_decimal::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum OperationToken {
    Subtract,
    Add,
    Multiply,
    Divide,
    FractionDivide,
    Pow,
    Root,
    LParent,
    RParent,
}

pub struct OperatorInfo {
    // how many operands
    arity: u8,
    // determines which operation is performed first (bigger -> priority)
    precedence: i8,
    // whether the order of the operands changes the result
    orderless: bool,
    // associativity_left: bool,
}

impl OperationToken {
    pub fn info(&self) -> OperatorInfo {
        match self {
            OperationToken::Add => OperatorInfo {
                arity: 2,
                precedence: 1,
                orderless: true,
            },
            OperationToken::Subtract => OperatorInfo {
                arity: 2,
                precedence: 1,
                orderless: false,
            },
            OperationToken::Multiply => OperatorInfo {
                arity: 2,
                precedence: 2,
                orderless: true,
            },
            OperationToken::Divide => OperatorInfo {
                arity: 2,
                precedence: 2,
                orderless: false,
            },
            OperationToken::Pow | OperationToken::Root => OperatorInfo {
                arity: 2,
                precedence: 3,
                orderless: false,
            },
            OperationToken::FractionDivide => todo!(),
            OperationToken::LParent | OperationToken::RParent => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MathToken {
    Constant(Decimal),
    Variable(String),
    Op(OperationToken),
}
