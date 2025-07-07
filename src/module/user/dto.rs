use chrono::NaiveDate;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[allow(dead_code)]
fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new(
            "Password must contain at least one number",
        ));
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter",
        ));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 20 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 20,
        message = "Name must be between 8 and 20 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 20 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 8,
        max = 20,
        message = "Name must be between 8 and 20 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DeleteUserDTO {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 20 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 8,
        max = 20,
        message = "Password must be between 8 and 20 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePasswordDTO {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 20 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 8,
        max = 20,
        message = "Password must be between 8 and 20 characters"
    ))]
    pub password: String,

    #[validate(length(
        min = 8,
        max = 20,
        message = "New password must be between 8 and 20 characters"
    ))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PatchInfoDTO {
    #[serde(deserialize_with = "deserialize_naive_date")] // Custom deserializer
    #[validate(custom(function = "validate_dob"))]
    pub dob: Option<NaiveDate>,

    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 20 characters"
    ))]
    pub role: Option<String>,

    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 characters"
    ))]
    pub address: Option<String>,
}

/// Custom deserializer for NaiveDate
fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(date_str) => NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

/// Adjusted validator function
fn validate_dob(dob: &&NaiveDate) -> Result<(), validator::ValidationError> {
    let today = chrono::Utc::now().naive_utc().date();
    if **dob > today {
        return Err(validator::ValidationError::new(
            "Date of birth must be in the past",
        ));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct PutInfoDTO {
    #[validate(required)]
    #[serde(deserialize_with = "deserialize_naive_date")] // Custom deserializer
    #[validate(custom(function = "validate_dob"))]
    pub dob: Option<NaiveDate>,

    #[validate(required)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 20 characters"
    ))]
    pub role: Option<String>,

    #[validate(required)]
    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 characters"
    ))]
    pub address: Option<String>,
}
