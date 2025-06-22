use thiserror::Error;
// Minimum length of the password
pub const MIN_PASSWORD_LENGTH: usize = 8;
// Maximum length of the password
pub const MAX_PASSWORD_LENGTH: usize = 16;

/// This is an enum that represents the possible errors that can occur
/// when validating a password.
#[derive(Debug, Error, PartialEq)]
pub enum PasswordErrors {
    #[error("Password must be at least {MIN_PASSWORD_LENGTH} characters long!")]
    TooShort,
    #[error("Password must be at most {MAX_PASSWORD_LENGTH} characters long!")]
    TooLong,
    #[error("Password is missing a lowercase letter!")]
    MissingLowerCase,
    #[error("Password is missing an uppercase letter!")]
    MissingUpperCase,
    #[error("Password is missing a digit!")]
    MissingDigit,
}
