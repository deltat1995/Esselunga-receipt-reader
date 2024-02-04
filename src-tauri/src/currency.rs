use core::fmt;
use std::{
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Default, Clone)]
pub struct Money {
    negative: bool,
    euro: u32,
    cents: u32,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.negative { "-" } else { "" };
        let zero = if self.cents < 10 { "0" } else { "" };
        write!(f, "{}{}.{}{}", sign, self.euro, zero, self.cents)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMoneyError;

impl FromStr for Money {
    type Err = ParseMoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new_str = s;
        let mut negative = false;
        if s.starts_with("-") {
            negative = true;
            new_str = new_str.strip_prefix("-").unwrap()
        }
        let (euro, cents) = new_str.trim().split_once('.').ok_or(ParseMoneyError)?;

        Ok(Money {
            negative: negative,
            euro: euro.parse().map_err(|_| ParseMoneyError)?,
            cents: cents.parse().map_err(|_| ParseMoneyError)?,
        })
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, other: Self) -> Self::Output {
        let cent1 = (if self.negative { -1 } else { 1 }) * (self.euro * 100 + self.cents) as i32;
        let cent2: i32 =
            (if other.negative { -1 } else { 1 }) * (other.euro * 100 + other.cents) as i32;

        let cent_tot = cent1 + cent2;

        let new_cents: u32 = cent_tot.abs() as u32 % 100;
        let new_euro = cent_tot.abs() as u32 / 100;

        Money {
            negative: cent_tot < 0,
            euro: new_euro,
            cents: new_cents,
        }
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        let cent1 = (if self.negative { -1 } else { 1 }) * (self.euro * 100 + self.cents) as i32;
        let cent2: i32 =
            (if other.negative { -1 } else { 1 }) * (other.euro * 100 + other.cents) as i32;

        let cent_tot = cent1 + cent2;

        let new_cents: u32 = cent_tot.abs() as u32 % 100;
        let new_euro = cent_tot.abs() as u32 / 100;

        *self = Money {
            negative: cent_tot < 0,
            euro: new_euro,
            cents: new_cents,
        }
    }
}
