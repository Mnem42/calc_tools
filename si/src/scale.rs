use std::{fmt::Display, ops::{Div,Mul,Neg}, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIScale {
    Unit,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto
}

impl SIScale{
    pub fn get_scale_factor(self) -> f64{
        f64::powi(10.0, match self {
            Self::Unit  => 0,
            Self::Milli => -3,
            Self::Micro => -6,
            Self::Nano  => -9,
            Self::Pico  => -12,
            Self::Femto => -15
        })
    }

    pub fn get_inverse_scale_factor(self) -> f64{
        f64::powi(10.0, match self {
            Self::Unit  => 0,
            Self::Milli => 3,
            Self::Micro => 6,
            Self::Nano  => 9,
            Self::Pico  => 12,
            Self::Femto => 15
        })
    }

    pub fn shift_up(self) -> Self{
        match self{
            Self::Unit  => Self::Unit,
            Self::Milli => Self::Unit,
            Self::Micro => Self::Milli,
            Self::Nano  => Self::Micro,
            Self::Pico  => Self::Nano,
            Self::Femto => Self::Pico
        }
    }

    pub fn shift_down(self) -> Self{
        match self{
            Self::Unit  => Self::Milli,
            Self::Milli => Self::Micro,
            Self::Micro => Self::Nano,
            Self::Nano  => Self::Pico,
            Self::Pico  => Self::Femto,
            Self::Femto => Self::Femto
        }
    }
}

impl Display for SIScale{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Unit  => "",
            Self::Milli => "m",
            Self::Micro => "u",
            Self::Nano  => "n",
            Self::Pico  => "p",
            Self::Femto => "f"
        })
    }
}

impl Neg for SIScale{
    type Output = f64;

    fn neg(self) -> Self::Output {
           self.get_inverse_scale_factor()
    }
}

impl Neg for &SIScale{
    type Output = f64;
    
    fn neg(self) -> Self::Output {
        -*self
    }
}

impl Mul<f64> for SIScale{
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        let converted: f64 = self.get_scale_factor();
        rhs * converted
    }
}
impl Mul<&f64> for &SIScale{
    type Output = f64;

    fn mul(self, rhs: &f64) -> Self::Output {
        let converted: f64 = self.get_scale_factor();
        rhs * converted
    }
}
impl Mul<SIScale> for f64{
    type Output = f64;

    fn mul(self, rhs: SIScale) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for SIScale{
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        let converted: f64 = self.get_scale_factor();
        rhs / converted
    }
}
impl Div<&f64> for &SIScale{
    type Output = f64;

    fn div(self, rhs: &f64) -> Self::Output {
        let converted: f64 = self.get_scale_factor();
        rhs / converted
    }
}

impl FromStr for SIScale{
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0){
            Some('m') => Ok(Self::Milli),
            Some('u') => Ok(Self::Micro),
            Some('p') => Ok(Self::Pico),
            Some('n') => Ok(Self::Nano),
            Some('f') => Ok(Self::Femto),
            Some(x)   => Err(x),
            None      => Ok(Self::Unit)
        }
    }
}

impl From<char> for SIScale{
    fn from(s: char) -> Self {
        match s{
            'm' => Self::Milli,
            'u' => Self::Micro,
            'p' => Self::Pico,
            'n' => Self::Nano,
            'f' => Self::Femto,
            _ => Self::Unit
        }
    }
}