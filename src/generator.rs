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
