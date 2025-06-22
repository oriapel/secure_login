//! This module provides a function to validate passwords based on specific requirements.

use crate::password_errors::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH, PasswordErrors, Result};

/// This function validates a password based on the following rules:
/// 1. The password must be at least [`MIN_PASSWORD_LENGTH`] characters long.
/// 2. The password must not exceed [`MAX_PASSWORD_LENGTH`] characters.
/// 3. The password must contain at least one lowercase letter.
/// 4. The password must contain at least one uppercase letter.
/// 5. The password must contain at least one digit.
///
pub fn validate_password(password: &str) -> Result<()> {
    let pswd_len = password.len();
    // First let us validate the length, before anything else, as it
    // does not require us to iterate the password, which makes it the "easiest"
    if pswd_len < MIN_PASSWORD_LENGTH {
        return Err(PasswordErrors::TooShort);
    }
    if pswd_len > MAX_PASSWORD_LENGTH {
        return Err(PasswordErrors::TooLong);
    }

    // This code is not really efficient as it it iterating the password more than one time
    // It does not really matter here, but if we cared about performance here, we could unite it all
    // to one loop so we only iterate the password once
    if !(password.chars().any(|c| c.is_lowercase())) {
        return Err(PasswordErrors::MissingLowerCase);
    }

    if !(password.chars().any(|c| c.is_uppercase())) {
        return Err(PasswordErrors::MissingUpperCase);
    }

    if !(password.chars().any(|c| c.is_ascii_digit())) {
        return Err(PasswordErrors::MissingDigit);
    }

    Ok(())
}
