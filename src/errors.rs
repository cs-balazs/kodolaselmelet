use std::fmt::Display;

#[derive(Debug)]
pub struct NotPrimeError;

impl Display for NotPrimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NotPrimeError")
    }
}

impl std::error::Error for NotPrimeError {}
