pub const SYMBOLS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const DIGITS: &str = "0123456789";

#[derive(Clone, Copy, Debug)]
pub enum PasswordCharRule {
    Symbols,
    Lower,
    Upper,
    Digit,
}

#[derive(Clone, Copy, Debug)]
pub enum InsertDirection {
    Front,
    Back,
}
