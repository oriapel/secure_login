use thiserror::Error;
#[derive(Debug, Error, PartialEq)]
pub enum PasswordErrors {
    #[error("Password is too short!")]
    TooShort,
    #[error("Password is too long!")]
    TooLong,
    #[error("Paswword is missing an lowercase letter!")]
    MissingLowerCase,
    #[error("Password is missing an uppercase letter!")]
    MissingUpperCase,
    #[error("Password is missing a digit!")]
    MissingDigit,
}
