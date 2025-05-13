use std::{fmt::Display, io::{stdout, Write}};

use text_io::try_read;
use uom::si::f32::Length;

/// A warning returned by a calculator.
pub struct CalcWarning{
    /// Warning text.
    pub text: String
}

impl Display for CalcWarning{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<&str> for CalcWarning{
    fn from(s: &str) -> Self {
        Self{
            text: s.to_string()
        }
    }
}

/// A returned value from a calculator. This also handles display and input of values
#[derive(Clone, Debug, PartialEq)]
pub enum Value{
    /// String value
    String(String, String),
    /// Decimal value
    Numeric(f64, String),
    /// SI unit
    Length(Length, String)
}

impl Value{
    /// Query as an argument
    pub fn query_arg(&self) -> Result<Value, text_io::Error>{
        match self {
            Value::Numeric(_, name) =>{
                print!("{}: ", name);
                stdout().flush().unwrap();

                Ok(Value::Numeric(try_read!()?, name.clone()))
            },
            Value::String(_, name) => {
                print!("{}: ", name);
                stdout().flush().unwrap();

                Ok(Value::String(try_read!()?, name.clone()))
            },
            Value::Length(_, name) => {
                print!("{}: ", name);
                stdout().flush().unwrap();
                
                Ok(Value::Length(try_read!()?, name.clone()))
            }
        }
    }
}

impl Display for Value{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Numeric(val, name) => write!(f, "{name}: {val:.4}"),
            Self::String(val, name) => write!(f, "{name}: {val}"),
            Self::Length(val, name) => {
                write!(f,"{name}: {:#?}",
                    val
                )
            }
        }
    }
}
