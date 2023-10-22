use std::collections::VecDeque;

use crate::models::InsertDirection;
pub use crate::models::{PasswordCharRule, DIGITS, LOWERCASE, SYMBOLS, UPPERCASE};
pub struct Generator {
    lower: Vec<char>,
    upper: Vec<char>,
    digit: Vec<char>,
    symbol: Vec<char>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            // creates character arrays for simpler use
            lower: LOWERCASE.chars().collect(),
            upper: UPPERCASE.chars().collect(),
            digit: DIGITS.chars().collect(),
            symbol: SYMBOLS.chars().collect(),
        }
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
        &self,
        len: u8,
        with_symbols: bool,
        with_numbers: bool,
        with_uppercase: bool,
        with_lowercase: bool,
    ) -> String {
        let password_rules = Self::generate_password_rules(
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
            match Self::get_random_element(&[InsertDirection::Back, InsertDirection::Front]) {
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
        &self,
        password_rules: impl Iterator<Item = &'a PasswordCharRule>,
    ) -> String {
        password_rules
            .map(|rule| match rule {
                PasswordCharRule::Upper => Self::get_random_element(&self.upper),
                PasswordCharRule::Lower => Self::get_random_element(&self.lower),
                PasswordCharRule::Digit => Self::get_random_element(&self.digit),
                PasswordCharRule::Symbols => Self::get_random_element(&self.symbol),
            })
            .collect::<String>()
    }

    /// Selects a random element from the array provided
    /// using a Criptographically secure pseudo rng to determine the index
    ///
    /// ## Arguments
    /// - `elements` the array of elements to select from
    ///
    /// ## Returns
    /// A single element from the array
    pub fn get_random_element<T: Clone>(_elements: &[T]) -> T {
        unimplemented!()
    }
}
