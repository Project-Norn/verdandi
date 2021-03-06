use crate::{
    common::{
        operator::{BinaryOperator, UnaryOperator},
        pos::Pos,
        types::Type,
    },
    frontend::lexer::token::TokenKind,
};
use std::{error, fmt};

#[derive(Debug)]
pub enum ErrorKind {
    // lexer
    UnexpectedChar {
        c: char,
    },

    // parser
    UnexpectedToken {
        expected: Option<TokenKind>,
        actual: TokenKind,
    },
    ExpectedIdent {
        actual: TokenKind,
    },
    NotTypeName {
        name: String,
    },

    // pass
    MainNotFound,
    MainShouldReturnInt,
    TypeMismatch {
        lhs: Type,
        rhs: Type,
    },
    CannotIndex {
        lhs: Type,
    },
    CannotLoad {
        lhs: Type,
    },
    AssignToConstant {
        name: String,
    },
    NotDefinedVariable {
        name: String,
    },
    NotDefinedFunction {
        name: String,
    },
    UnaryOpErr {
        op: UnaryOperator,
        expr: Type,
    },
    BinaryOpErr {
        op: BinaryOperator,
        lhs: Type,
        rhs: Type,
    },
    FunctionArgNum {
        name: String,
        expected: usize,
        actual: usize,
    },
    RedefinitionOf {
        name: String,
    },
    LvalueRequired,

    // regalloc
    RegistersExhausted,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self {
            UnexpectedChar { ref c } => write!(f, "unexpected char '{}'", c),

            UnexpectedToken {
                ref expected,
                ref actual,
            } => {
                write!(f, "unexpected {:?}", actual)?;
                if let Some(expected) = expected {
                    write!(f, ", expecting {:?}", expected)?;
                }
                Ok(())
            }
            ExpectedIdent { ref actual } => write!(f, "expected identifier, but got {:?}", actual),
            NotTypeName { ref name } => write!(f, "'{}' is not a type name", name),

            MainNotFound => write!(f, "there must be 'main' function"),
            MainShouldReturnInt => write!(f, "'main' function should return int value"),
            TypeMismatch { ref lhs, ref rhs } => write!(f, "type mismatch {} and {}", lhs, rhs),
            CannotIndex { ref lhs } => write!(f, "cannot index type {}", lhs),
            CannotLoad { ref lhs } => write!(f, "cannot load type {}", lhs),
            AssignToConstant { ref name } => {
                write!(f, "cannot assign to constant variable '{}'", name)
            }
            NotDefinedVariable { ref name } => write!(f, "undefined variable '{}'", name),
            NotDefinedFunction { ref name } => write!(f, "undefined function '{}'", name),
            UnaryOpErr { ref op, ref expr } => write!(f, "cannot {:?} {}", op, expr),
            BinaryOpErr {
                ref op,
                ref lhs,
                ref rhs,
            } => write!(f, "cannot {} {:?} {}", lhs, op, rhs),
            FunctionArgNum {
                name,
                expected,
                actual,
            } => write!(
                f,
                "'{}' function takes {} argument but {} arguments were supplied",
                name, expected, actual
            ),
            RedefinitionOf { name } => write!(f, "redefinition of '{}'", name),
            LvalueRequired => write!(f, "lvalue required"),
            RegistersExhausted => write!(f, "registers exhausted"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

impl Error {
    pub fn new(pos: Pos, kind: ErrorKind) -> Self {
        Self { pos, kind }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.pos, self.kind)
    }
}

impl error::Error for Error {}

#[derive(Debug, Default)]
pub struct Errors(pub Vec<Error>);

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for err in self.0.iter() {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl error::Error for Errors {}
