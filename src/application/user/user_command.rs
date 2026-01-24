use chrono::NaiveDate;
use validator::Validate;

#[derive(Debug, Clone)]
pub struct VerifyEmailCommand {
    pub verification_token: String,
}

#[derive(Debug, Clone, Validate)]
pub struct ResendVerificationEmailCommand {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Validate)]
pub struct RegisterUserCommand {
    #[validate(length(
        min = 8,
        max = 25,
        message = "Passwords must be between 8 and 25 characters long"
    ))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(
        min = 2,
        max = 30,
        message = "The first name must be between 2 and 30 characters long"
    ))]
    pub first_name: String,
    #[validate(length(
        min = 2,
        max = 30,
        message = "The last name must be between 2 and 30 characters long"
    ))]
    pub last_name: String,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Validate)]
pub struct AdminCreateUserCommand {
    #[validate(email)]
    pub email: String,
    #[validate(length(
        min = 2,
        max = 30,
        message = "The first name must be between 2 and 30 characters long"
    ))]
    pub first_name: String,
    #[validate(length(
        min = 2,
        max = 30,
        message = "The last name must be between 2 and 30 characters long"
    ))]
    pub last_name: String,
    pub birth_of_date: Option<NaiveDate>,
    #[validate(url)]
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Validate)]
pub struct UpdateUserCommand {
    #[validate(length(min = 2, max = 30))]
    pub first_name: Option<String>,
    #[validate(length(min = 2, max = 30))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(url)]
    pub avatar: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Validate)]
pub struct CreateUserCommand {
    #[validate(length(min = 2, max = 30))]
    pub first_name: String,
    #[validate(length(min = 2, max = 30))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(url)]
    pub avatar: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
    #[validate(length(
        min = 8,
        max = 25,
        message = "Passwords must be between 8 and 25 characters long"
    ))]
    pub password: String,
}
