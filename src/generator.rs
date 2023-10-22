use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::VecDeque;

use crate::models::InsertDirection;
pub use crate::models::{PasswordCharRule, DIGITS, LOWERCASE, SYMBOLS, UPPERCASE};
pub struct Generator {
    lower: Vec<char>,
    upper: Vec<char>,
    digit: Vec<char>,
    symbol: Vec<char>,
    rng: StdRng,
}

impl Generator {
    pub fn new() -> Self {
        // The random number generator should use the OS entropy for more secure generation
        let rng = StdRng::from_entropy();
        Self {
            // creates character arrays for simpler use
            lower: LOWERCASE.chars().collect(),
            upper: UPPERCASE.chars().collect(),
            digit: DIGITS.chars().collect(),
            symbol: SYMBOLS.chars().collect(),
            rng,
        }
    }

    /// Calculates the strength of the password
    /// based on the rules applied to the password
    /// and it's length
    ///
    /// ## Arguments
    /// - `len` the password length
    /// - `with_symbols` whether to include symbols
    /// - `with_digits` whether to include digits
    /// - `with_uppercase` whether to include uppercase characters
    /// - `with_lowercase` whether to include lowercase characters
    ///
    /// ## Returns
    /// the password strength percentage
    pub fn get_password_strength(
        &self,
        len: u8,
        with_symbols: bool,
        with_numbers: bool,
        with_uppercase: bool,
        with_lowercase: bool,
    ) -> f64 {
        if len == 0 {
            return 0.0_f64;
        } else if len == 1 {
            return 0.0_f64;
        }
        let max_multiplier: f64 = 32.0 + 10.0 + 26.0 + 26.0; // all possible password options

        // this password options used
        let mut multiplier: f64 = 0.0;
        if with_symbols {
            multiplier += 32.0;
        }
        if with_numbers {
            multiplier += 10.0;
        }
        if with_uppercase {
            multiplier += 26.0;
        }
        if with_lowercase {
            multiplier += 26.0;
        }
        // The possible combinations of passwords
        let a = (len as f64).powf(multiplier);
        let b: f64 = (255.0_f64).powf(max_multiplier);
        // wanted to increase the baseline to a min of 20% for low strength
        20.0 + (a.log(b) * 80.0)
    }

    /// Generates a password with the provided rules
    ///
    /// ## Arguments
    /// - `len` the length of the password
    /// - `with_symbols` whether to include symbols
    /// - `with_digits` whether to include digits
    /// - `with_uppercase` whether to include uppercase characters
    /// - `with_lowercase` whether to include lowercase characters
    ///
    /// ## Returns
    /// The generated password
    pub fn generate_password(
        &mut self,
        len: u8,
        with_symbols: bool,
        with_numbers: bool,
        with_uppercase: bool,
        with_lowercase: bool,
    ) -> String {
        let password_rules = self.generate_password_rules(
            len,
            with_symbols,
            with_numbers,
            with_uppercase,
            with_lowercase,
        );
        self.fill_password(password_rules.iter())
    }

    /// Creates a collection of password character rules
    /// based on the password rules that are provided
    /// in a random order
    ///
    /// ## Arguments
    /// - `len` the length of the password
    /// - `with_symbols` whether to include symbols
    /// - `with_digits` whether to include digits
    /// - `with_uppercase` whether to include uppercase characters
    /// - `with_lowercase` whether to include lowercase characters
    ///
    /// ## Returns
    /// A randomly ordered collection of password character rules
    fn generate_password_rules(
        &mut self,
        mut len: u8,
        with_symbols: bool,
        with_numbers: bool,
        with_uppercase: bool,
        with_lowercase: bool,
    ) -> VecDeque<PasswordCharRule> {
        let mut distributed_rules = VecDeque::new();
        if with_symbols {
            distributed_rules.push_back(PasswordCharRule::Symbols);
        }
        if with_numbers {
            distributed_rules.push_back(PasswordCharRule::Digit);
        }
        if with_lowercase {
            distributed_rules.push_back(PasswordCharRule::Lower);
        }
        if with_uppercase {
            distributed_rules.push_back(PasswordCharRule::Upper);
        }
        // This ensures that all rules are inserted evenly (ex: 5 lowercase and 1 symbol will not happen)
        let mut password_char_rules_unsorted = Vec::new();
        while len > 0 {
            if let Some(next) = distributed_rules.pop_front() {
                password_char_rules_unsorted.push(next);
                distributed_rules.push_back(next);
            }
            len -= 1;
        }

        // Random sorting of the password rules
        let mut password_char_rules = VecDeque::new();
        for rule in password_char_rules_unsorted {
            match self.get_random_element(&[InsertDirection::Back, InsertDirection::Front]) {
                InsertDirection::Back => password_char_rules.push_back(rule),
                InsertDirection::Front => password_char_rules.push_front(rule),
            }
        }
        password_char_rules
    }

    /// Using the password char rule collection,
    /// A random element will be selected and used for the password
    ///
    /// ## Arguments
    /// - `char_rules` An iterator over Password char rules to fill
    ///
    /// ## Returns
    /// A complete password
    fn fill_password<'a>(
        &mut self,
        password_rules: impl Iterator<Item = &'a PasswordCharRule>,
    ) -> String {
        let lower = self.lower.clone();
        let upper = self.upper.clone();
        let digit = self.digit.clone();
        let symbol = self.symbol.clone();
        password_rules
            .map(move |rule| match rule {
                PasswordCharRule::Upper => self.get_random_element(&upper),
                PasswordCharRule::Lower => self.get_random_element(&lower),
                PasswordCharRule::Digit => self.get_random_element(&digit),
                PasswordCharRule::Symbols => self.get_random_element(&symbol),
            })
            .collect::<String>()
    }

    /// Selects a random element from the array provided
    /// using a Criptographically secure pseudo rng to determine the index
    /// https://rust-random.github.io/rand/rand/rngs/struct.StdRng.html
    ///
    /// ## Arguments
    /// - `elements` the array of elements to select from
    ///
    /// ## Returns
    /// A single element from the array
    pub fn get_random_element<T: Clone>(&mut self, elements: &[T]) -> T {
        let i = self.rng.gen_range(0..elements.len());
        elements[i].clone()
    }
}

#[test]
fn generate_different_sizes() {
    let mut generator = Generator::new();
    for i in 0..255 {
        let password = generator.generate_password(i, true, true, true, true);
        assert_eq!(password.len() as u8, i, "Password is not the right length");
    }
}

#[test]
fn generate_pin() {
    let mut generator = Generator::new();
    let password = generator.generate_password(5, false, true, false, false);
    assert_eq!(password.len(), 5, "Password is not the right length");
    for c in password.chars() {
        assert!(
            crate::models::DIGITS.contains(c),
            "Password has something other than a digit"
        );
    }
}

#[test]
fn generate_random() {
    let mut generator = Generator::new();
    let password = generator.generate_password(10, true, true, true, true);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, true, true, true, true);
}

#[test]
fn generate_with_two_rules() {
    let mut generator = Generator::new();
    let password = generator.generate_password(10, true, true, false, false);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, true, true, false, false);

    let password = generator.generate_password(10, true, false, true, false);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, true, false, true, false);

    let password = generator.generate_password(10, true, false, false, true);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, true, false, false, true);

    let password = generator.generate_password(10, false, true, true, false);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, false, true, true, false);

    let password = generator.generate_password(10, false, false, true, true);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, false, false, true, true);
}

#[test]
fn generate_with_lower_upper_digit_rules() {
    let mut generator = Generator::new();
    let password = generator.generate_password(10, false, true, true, true);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, false, true, true, true);
}

#[test]
fn generate_with_symbol_lower_upper_rules() {
    let mut generator = Generator::new();
    let password = generator.generate_password(10, true, false, true, true);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, true, false, true, true);
}

#[test]
fn generate_with_symbol_lower_digit_rules() {
    let mut generator = Generator::new();
    let password = generator.generate_password(10, true, true, false, true);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, true, true, false, true);
}

#[test]
fn generate_with_symbol_upper_digit_rules() {
    let mut generator = Generator::new();
    let password = generator.generate_password(10, true, true, true, false);
    assert_eq!(password.len(), 10, "Password is not the right length");
    assert_password(&password, true, true, true, false);
}

#[test]
fn generate_unique_passwords() {
    let mut generator = Generator::new();
    let mut previously_generated_passwords = std::collections::HashSet::new();
    for _ in 0..100000 {
        let password = generator.generate_password(10, true, true, true, false);
        assert_eq!(password.len(), 10, "Password is not the right length");
        assert_password(&password, true, true, true, false);
        assert!(
            !previously_generated_passwords.contains(password.as_str()),
            "The password has already been generated"
        );
        previously_generated_passwords.insert(password);
    }
}

#[cfg(test)]
/// Helper function to assert a password and it's rules
/// Verifies that at least one character of each applied rule is included
/// Then asserts the requested rules with the ones that were found in the password
///
/// ## Arguments
/// - `password` The password being asserted
/// - `should_have_symbol` whether a symbol should exist in the password
/// - `should_have_number` whether a number should exist in the password
/// - `should_have_upper` whether a uppercase letter should exist in the password
/// - `should_have_lower` whether a lowercase letter should exist in the password
fn assert_password(
    password: &str,
    should_have_symbol: bool,
    should_have_number: bool,
    should_have_upper: bool,
    should_have_lower: bool,
) {
    let mut has_symbol = false;
    let mut has_number = false;
    let mut has_lower = false;
    let mut has_upper = false;
    for c in password.chars() {
        has_symbol = has_symbol || crate::models::SYMBOLS.contains(c);
        has_number = has_number || crate::models::DIGITS.contains(c);
        has_lower = has_lower || crate::models::LOWERCASE.contains(c);
        has_upper = has_upper || crate::models::UPPERCASE.contains(c);
    }

    assert!(
        !(should_have_symbol ^ has_symbol)
            && !(should_have_number ^ has_number)
            && !(should_have_lower ^ has_lower)
            && !(should_have_upper ^ has_upper)
    );
}
