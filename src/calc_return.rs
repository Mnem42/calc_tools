use std::{fmt::Display, io::{stdout, Write}};

use crate::si::si::SINumber;
use text_io::try_read;

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
    SI(SINumber, String)
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
            Value::SI(default, name) => {
                print!("{}: ", name);
                stdout().flush().unwrap();
                let str: String = try_read!()?;

                match SINumber::from_str(&str, default.unit){
                    Ok(x) => Ok(Value::SI(x, name.clone())),
                    Err(_) => Ok(Value::SI(*default, name.clone()))
                }
            }
        }
    }
}

impl Display for Value{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Numeric(val, name) => write!(f, "{name}: {val:.4}"),
            Self::String(val, name) => write!(f, "{name}: {val}"),
            Self::SI(val, name) => {
                let mut tmp = *val;
                tmp.adjust_scale();
                write!(f,"{name}: {:.3}{}{}",
                    -tmp.scale * val.val,
                    tmp.scale,
                    tmp.unit
                )
            }
        }
    }
}
