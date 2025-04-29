use crate::calc_return::{CalcWarning, Value};

/// Struct for calculator info.
pub struct CalcInfo{
    /// Title
    pub title: &'static str,

    /// Description
    pub description: &'static str
}

/// Calculator trait. This has to be defined for a calculators to be runnable with
/// this crate.
/// 
/// Calculators should *not* use IO directly, because this can cause problems when
/// they're actually used.
pub trait Calculator {
    /// Run calculator. Takes a set of args, and returns warnings and return values.
    /// 
    /// Calculators should indicate recoverable errors with `CalcWarnings`, and only
    /// return `Err()` if there's an uncrecoverable error at some point.
    fn calc(&self, vec: Vec<Value>) -> Result<(Vec<Value>,Vec<CalcWarning>),String>;

    /// Get an array of all arguments, with defaults. This is used for querying arguments
    /// to the calculator.
    fn get_signature(&self) -> Vec<Value>;

    /// Get calculator info, like name and title.
    fn get_info(&self) -> CalcInfo;
}
