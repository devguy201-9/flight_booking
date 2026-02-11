use crate::application::user::user_command::{
    AdminCreateUserCommand, CreateUserCommand, RegisterUserCommand, ResendVerificationEmailCommand,
    UpdateUserCommand, VerifyEmailCommand,
};
use crate::presentation::user::user_request::{
    AdminCreateUserRequest, CreateUserRequest, RegisterUserRequest, ResendVerificationEmailRequest,
    UpdateUserRequest, VerifyEmailRequest,
};

impl From<CreateUserRequest> for CreateUserCommand {
    fn from(req: CreateUserRequest) -> Self {
        Self {
            first_name: req.first_name,
            last_name: req.last_name,
            email: req.email,
            avatar: req.avatar,
            birth_of_date: req.birth_of_date,
            phone_number: req.phone_number,
            gender: req.gender,
            password: req.password,
        }
    }
}
impl From<UpdateUserRequest> for UpdateUserCommand {
    fn from(req: UpdateUserRequest) -> Self {
        Self {
            avatar: req.avatar,
            first_name: req.first_name,
            last_name: req.last_name,
            email: req.email,
            birth_of_date: req.birth_of_date,
            phone_number: req.phone_number,
            gender: req.gender,
            version: req.version,
        }
    }
}

impl From<VerifyEmailRequest> for VerifyEmailCommand {
    fn from(req: VerifyEmailRequest) -> Self {
        Self {
            verification_token: req.verification_token,
        }
    }
}

impl From<ResendVerificationEmailRequest> for ResendVerificationEmailCommand {
    fn from(req: ResendVerificationEmailRequest) -> Self {
        Self { email: req.email }
    }
}

impl From<RegisterUserRequest> for RegisterUserCommand {
    fn from(req: RegisterUserRequest) -> Self {
        Self {
            password: req.password,
            email: req.email,
            first_name: req.first_name,
            last_name: req.last_name,
            birth_of_date: req.birth_of_date,
            phone_number: req.phone_number,
            gender: req.gender,
        }
    }
}

impl From<AdminCreateUserRequest> for AdminCreateUserCommand {
    fn from(req: AdminCreateUserRequest) -> Self {
        Self {
            email: req.email,
            first_name: req.first_name,
            last_name: req.last_name,
            birth_of_date: req.birth_of_date,
            avatar: req.avatar,
            phone_number: req.phone_number,
            gender: req.gender,
        }
    }
}
