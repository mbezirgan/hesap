use std::fmt::{Display, Formatter};

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

    pub fn add_number(&mut self, number: &str) {
        if self.string == "0" {
            self.string.clear();
        }
        self.string.push_str(number);
    }

    pub fn swap_sign(&mut self) {
        self.negative = !self.negative;
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn to_f64(&self) -> f64 {
        // Due to how the string is being created this CANNOT fail
        let value: f64 = self.string.parse().unwrap();
        let sign = if self.negative { -1.0 } else { 1.0 };
        value * sign
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

#[cfg(test)]
mod tests {
    use super::*;

    // note no add_numbers will be added to DisplayNumber as this is meant purely for the calculator
    fn add_235(num: &mut DisplayNumber) {
        num.add_number("2");
        num.add_number("3");
        num.add_number("5");
    }

    fn make_235() -> DisplayNumber {
        let mut num = DisplayNumber::default();
        add_235(&mut num);
        num
    }

    #[test]
    fn add_numbers() {
        let num = make_235();
        assert_eq!(num.to_string(), "235");
    }

    #[test]
    fn add_zero() {
        let mut num = DisplayNumber::default();
        assert_eq!(num.to_string(), "0");
        num.add_number("0");
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
    }
}