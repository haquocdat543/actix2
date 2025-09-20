use chrono::NaiveDate;
use serde::Deserializer;
use serde::{Deserialize, de::Error};
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

/// Corrected deserializer with proper trait bounds
fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>, // This constraint provides the Error type
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(date_str) => {
            // Try multiple common date formats
            let formats = [
                "%Y-%m-%d",  // ISO 8601
                "%m/%d/%Y",  // US format
                "%d.%m.%Y",  // European format
                "%Y%m%d",    // Compact format
                "%B %d, %Y", // "January 01, 2023"
            ];

            // Try each format until one succeeds
            for fmt in &formats {
                if let Ok(parsed) = NaiveDate::parse_from_str(&date_str, fmt) {
                    return Ok(Some(parsed));
                }
            }

            // If none worked, return a descriptive error
            Err(D::Error::custom(format!(
                "Invalid date format '{}'. Accepted formats: {}, {}, {}, {}, {}",
                date_str,
                "YYYY-MM-DD",
                "MM/DD/YYYY",
                "DD.MM.YYYY",
                "YYYYMMDD",
                "MonthName DD, YYYY"
            )))
        }
        None => Ok(None),
    }
}

/// Enhanced date validator with age range checking
fn validate_dob(dob: &&NaiveDate) -> Result<(), ValidationError> {
    let today = chrono::Local::now().naive_local().date();
    let dob_date = **dob;

    // Check if date is in future
    if dob_date > today {
        let mut err = ValidationError::new("date_future");
        err.message = Some("Date of birth cannot be in the future".into());
        return Err(err);
    }

    // Check minimum age (13 years)
    let min_age = today - chrono::Duration::days(4745); // ~13 years accounting for leap years
    if dob_date > min_age {
        let mut err = ValidationError::new("age_requirement");
        err.message = Some("You must be at least 13 years old".into());
        return Err(err);
    }

    // Check maximum age (120 years)
    let max_age = today - chrono::Duration::days(43800); // ~120 years
    if dob_date < max_age {
        let mut err = ValidationError::new("age_requirement");
        err.message = Some("Please enter a valid date of birth".into());
        return Err(err);
    }

    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct PutInfoDTO {
    #[validate(required(message = "dob must be not null"))]
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

pub enum InfoDTO {
    Patch(PatchInfoDTO),
    Put(PutInfoDTO),
}
