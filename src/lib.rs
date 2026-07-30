use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::ops::Neg;
use rust_decimal::prelude::*;

// Special type to make sure no matter what the internal data type used for calculations is
// The user input string will act like the user expects
pub struct DisplayNumber {
    string: String,
    negative: bool
}

impl DisplayNumber {
    pub fn clear(&mut self) {
        self.string.clear();
        self.string.push('0');
        self.negative = false;
    }

    #[must_use]
    pub fn digits_used(&self) -> usize {
        let fractional = usize::from(self.string.contains('.'));
        self.string.len() - fractional
    }

    pub fn be_fractional(&mut self) {
        if !self.string.contains('.') {
            self.string.push('.');
        }
    }

    /// # Panics
    ///
    /// Panics if `digit` is greater than `9`.
    pub fn add_digit(&mut self, digit: Digit) {
        if self.string == "0" {
            self.string.clear();
        }

        let ch = (digit.value() + b'0') as char;
        self.string.push(ch);
    }

    pub fn remove_char(&mut self) {
        self.string.pop();
        if self.string.is_empty() {
            self.string.push('0');
        }
    }

    pub fn swap_sign(&mut self) {
        self.negative = !self.negative;
    }

    // NOTE: no testing used as this uses Decimal libraries implementation
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn to_decimal(&self) -> Decimal {
        // Due to how the string is being created this CANNOT fail
        let value = Decimal::from_str(&self.string).unwrap();
        if self.negative { value.neg() } else { value }
    }

    pub fn set_decimal(&mut self, value: Decimal) {
        self.negative = value.is_sign_negative();
        let value = value.abs().normalize();

        self.string.clear();
        // write! doesn't fail on string
        write!(&mut self.string, "{value}").unwrap();
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn to_f64(&self) -> f64 {
        // Due to how the string is being created this CANNOT fail
        let value: f64 = self.string.parse().unwrap();
        if self.negative { value.neg() } else { value }
    }

    pub fn set_f64(&mut self, value: f64) {
        self.negative = value < 0.0;
        let value = value.abs();

        self.string.clear();
        // write! doesn't fail on string
        write!(&mut self.string, "{value}").unwrap();
    }
}

impl Default for DisplayNumber {
    fn default() -> Self {
        DisplayNumber {
            string: String::from('0'),
            negative: false
        }
    }
}

impl Display for DisplayNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            write!(f, "-{}", self.string)
        } else {
            write!(f, "{}", self.string)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Digit { Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine }

impl Digit {
    pub fn value(self) -> u8 {
        match self {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }

    pub fn value_as_str(self) -> &'static str {
        match self {
            Digit::Zero => "0",
            Digit::One => "1",
            Digit::Two => "2",
            Digit::Three => "3",
            Digit::Four => "4",
            Digit::Five => "5",
            Digit::Six => "6",
            Digit::Seven => "7",
            Digit::Eight => "8",
            Digit::Nine => "9",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // note no add_numbers will be added to DisplayNumber as this is meant purely for the calculator
    fn add_235(num: &mut DisplayNumber) {
        num.add_digit(Digit::Two);
        num.add_digit(Digit::Three);
        num.add_digit(Digit::Five);
    }

    fn make_235() -> DisplayNumber {
        let mut num = DisplayNumber::default();
        add_235(&mut num);
        num
    }

    #[test]
    fn add_digits() {
        let num = make_235();
        assert_eq!(num.to_string(), "235");
    }

    #[test]
    fn add_zero() {
        let mut num = DisplayNumber::default();
        assert_eq!(num.to_string(), "0");
        num.add_digit(Digit::Zero);
        assert_eq!(num.to_string(), "0");
    }

    #[test]
    fn change_sign() {
        let mut num = make_235();
        num.swap_sign();
        assert_eq!(num.to_string(), "-235");
        num.swap_sign();
        assert_eq!(num.to_string(), "235");
    }

    #[test]
    fn use_fraction() {
        let mut num = DisplayNumber::default();
        num.be_fractional();
        assert_eq!(num.to_string(), "0.");
        num.be_fractional();
        assert_eq!(num.to_string(), "0.");

        add_235(&mut num);
        assert_eq!(num.to_string(), "0.235");

        num.clear();
        add_235(&mut num);
        num.be_fractional();
        add_235(&mut num);
        assert_eq!(num.to_string(), "235.235");
    }

    #[test]
    fn remove_digits() {
        let mut num = make_235();
        num.remove_char();
        assert_eq!(num.to_string(), "23");
        num.be_fractional();
        num.remove_char();
        assert_eq!(num.to_string(), "23");
        num.be_fractional();
        assert_eq!(num.to_string(), "23.");
        num.remove_char();
        num.remove_char();
        num.remove_char();
        assert_eq!(num.to_string(), "0");
    }

    #[test]
    fn count_digits() {
        let mut num = DisplayNumber::default();
        add_235(&mut num);
        assert_eq!(num.digits_used(), 3);

        num.clear();
        add_235(&mut num);
        num.be_fractional();
        add_235(&mut num);
        assert_eq!(num.digits_used(), 6);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn to_f64() {
        let mut num = DisplayNumber::default();
        add_235(&mut num);
        assert_eq!(num.to_f64(), 235.0);

        num.clear();
        add_235(&mut num);
        num.be_fractional();
        add_235(&mut num);
        assert_eq!(num.to_f64(), 235.235);

        num.clear();
        num.be_fractional();
        assert_eq!(num.to_f64(), 0.0);
    }

    #[test]
    fn set_f64() {
        let mut num = DisplayNumber::default();
        num.set_f64(232.5);
        assert_eq!(num.to_string(), "232.5");

        num.set_f64(-32.5);
        assert_eq!(num.to_string(), "-32.5");

        num.set_f64(0.0);
        assert_eq!(num.to_string(), "0");
    }
}