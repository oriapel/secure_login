mod password_checker;
mod password_errors;

use crate::password_checker::validate_password;

fn main() {
    let result = validate_password("MyPassword1");
    match result {
        Ok(_) => println!("Password met all of the requirements! :)"),
        Err(e) => println!("{e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::password_errors::PasswordErrors;

    #[test]
    fn test_valid_password() {
        let result = validate_password("Valid1Password");
        assert!(result.is_ok());
    }

    #[test]
    fn test_too_short_password() {
        let result = validate_password("Short1");
        assert_eq!(result, Err(PasswordErrors::TooShort));
    }

    #[test]
    fn test_too_long_password() {
        let result = validate_password("ThisPasswordIsWayTooLong1");
        assert_eq!(result, Err(PasswordErrors::TooLong));
    }

    #[test]
    fn test_missing_lowercase() {
        let result = validate_password("MISSINGLOWERCASE");
        assert_eq!(result, Err(PasswordErrors::MissingLowerCase));
    }

    #[test]
    fn test_missing_uppercase() {
        let result = validate_password("missinguppercase");
        assert_eq!(result, Err(PasswordErrors::MissingUpperCase));
    }

    #[test]
    fn test_missing_digit() {
        let result = validate_password("MissingDigit");
        assert_eq!(result, Err(PasswordErrors::MissingDigit));
    }
}
