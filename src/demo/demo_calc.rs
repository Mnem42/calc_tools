use std::ops::Index;
use crate::r#impl::{CalcInfo, Calculator};
use crate::calc_return::{Value, CalcWarning};

/// A demo calculator. Does addition, subtraction, multiplication, and division.
/// It can be run with run_calc(). Note that the arguments are named "add", "sub",
/// "mlt", and "div".
pub struct DemoCalc;

impl Calculator for DemoCalc{
    fn get_signature(&self) -> Vec<Value>{
        vec![
            Value::Numeric(0.0, "A".to_string()),
            Value::Numeric(0.0, "B".to_string()),
            Value::String(String::new(), "Mode".to_string())
        ]
    }

    fn calc(&self, args: Vec<Value>) -> Result<(Vec<Value>,Vec<CalcWarning>),String>{
        if let (Value::Numeric(a,_), Value::Numeric(b,_), Value::String(mode, _)) = 
               (args.index(0), args.index(1), args.index(2))
        {
            let result= match mode.as_str(){
                "add" => a+b,
                "sub" => a-b,
                "mul" => a*b,
                "div" => a/b,
                _ => return Err("Invalid mode".to_string())
            };

            Ok((vec![Value::Numeric(result, "C".to_string())],vec![]))
        }
        else{
            Err(format!("Argument error. Args: {:?}", args))
        }
    }

    fn get_info(&self) -> crate::r#impl::CalcInfo {
        CalcInfo{
            title: "DEMO CALCULATOR",
            description: "A demo calculator. The modes are add, sub, mul, and div.\n"
        }
    }
}