use std::{fmt::Display, num::ParseFloatError, ops::{Div, Mul}, str::FromStr};

use crate::scale::SIScale;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIUnit{
    Farad,
    Second,
    Ohm,
    Metre
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SINumber{
    pub scale: SIScale,
    pub unit: SIUnit,
    pub val: f64
}

impl SINumber{
    /// Initialise new
    pub fn new(val: f64,scale: SIScale, unit: SIUnit) -> Self{
        SINumber { scale, unit, val }
    }

    /// Adjust scale
    pub fn adjust_scale(&mut self){
        loop{
            if -self.scale * self.val > 1000.0{
                if self.scale == self.scale.shift_up() {break}
                self.scale = self.scale.shift_up();
            }
            else if -self.scale * self.val < 1.0{
                if self.scale == self.scale.shift_down() {break}
                self.scale = self.scale.shift_down();
            }
            else{
                break;
            }
        }
    }

    pub fn from_str(s: &str, unit: SIUnit) -> Result<Self, ParseFloatError> {
        // Get suffix and numeric part
        let suffix: String = s.chars().rev()
                              .filter(|x| x.is_alphabetic())
                              .rev().collect();
        let val: f64 = s.chars()
                        .take_while(|x| x.is_ascii_digit() || *x == '.')
                        .collect::<String>().parse()?;

        // if there's no suffix, assume it's unit
        if !s.ends_with(|x: char| x.is_alphabetic()){
            return Ok(SINumber{
                scale: SIScale::Unit,
                unit,
                val
            })
        }

        // Edge case for metres vs millimetres
        if unit == SIUnit::Metre && suffix == "m" && !s.ends_with(|x: char| x.is_alphabetic()){
            return Ok(SINumber{
                scale: SIScale::Unit,
                unit,
                val
            });
        }

        // Get scale and return
        let scale = suffix.chars().nth(0)
                                   .unwrap_or('.').into();
        Ok(SINumber { scale, unit, val})
    }
}

impl Display for SIUnit{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Farad  => "F",
            Self::Ohm    => "o",
            Self::Second => "s",
            Self::Metre  => "m"
        })
    }
}

impl FromStr for SIUnit{
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let last_char = s.char_indices().nth_back(0).unwrap().1;
        match last_char{
            'f' | 'F' => Ok(Self::Farad),
            'o'       => Ok(Self::Ohm),
            's' | 'S' => Ok(Self::Second),
            x => Err(x)
        }
    }
}

impl Mul<f64> for SINumber{
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scale * self.val * rhs
    }
}

impl Mul<f64> for &SINumber{
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scale * self.val * rhs
    }
}

impl Div<f64> for SINumber{
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.scale / self.val * rhs
    }
}

impl Div<f64> for &SINumber{
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.scale / self.val * rhs
    }
}

impl Div<SINumber> for f64{
    type Output = f64;

    fn div(self, rhs: SINumber) -> Self::Output {
        self / (rhs.val * rhs.scale)
    }
}
