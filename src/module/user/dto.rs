use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}
