use std::collections::VecDeque;

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
        _len: u8,
        _with_symbols: bool,
        _with_numbers: bool,
        _with_uppercase: bool,
        _with_lowercase: bool,
    ) -> VecDeque<PasswordCharRule> {
        unimplemented!()
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
