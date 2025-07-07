use serde::Deserialize;
use validator::Validate;

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
        message = "Name must be between 8 and 20 characters"
    ))]
    pub password: String,
}
