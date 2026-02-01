use crate::domain::user::entity::{User, UserRole, UserStatus};
use crate::infrastructure::persistence::seaorm::entities::user as user_orm;
use sea_orm::ActiveValue::{NotSet, Set};

pub struct UserMapper;

impl From<UserStatus> for user_orm::Status {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Pending => user_orm::Status::PENDING,
            UserStatus::Active => user_orm::Status::ACTIVE,
            UserStatus::Suspended => user_orm::Status::SUSPENDED,
            UserStatus::Deleted => user_orm::Status::DELETED,
        }
    }
}

impl From<UserRole> for user_orm::Role {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Customer => user_orm::Role::CUSTOMER,
            UserRole::Admin => user_orm::Role::ADMIN,
            UserRole::Staff => user_orm::Role::STAFF,
        }
    }
}

impl From<user_orm::Status> for UserStatus {
    fn from(status: user_orm::Status) -> Self {
        match status {
            user_orm::Status::PENDING => UserStatus::Pending,
            user_orm::Status::ACTIVE => UserStatus::Active,
            user_orm::Status::SUSPENDED => UserStatus::Suspended,
            user_orm::Status::DELETED => UserStatus::Deleted,
        }
    }
}

impl From<user_orm::Role> for UserRole {
    fn from(role: user_orm::Role) -> Self {
        match role {
            user_orm::Role::CUSTOMER => UserRole::Customer,
            user_orm::Role::ADMIN => UserRole::Admin,
            user_orm::Role::STAFF => UserRole::Staff,
        }
    }
}

impl UserMapper {
    pub fn domain_to_active_model_create(user: &User) -> user_orm::ActiveModel {
        user_orm::ActiveModel {
            id: NotSet,
            email: Set(user.email.clone()),
            username: Set(user.username.clone()),
            password_hash: Set(user.password_hash.clone()),

            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            avatar: Set(user.avatar.clone()),
            phone_number: Set(user.phone_number.clone()),
            birth_of_date: Set(user.birth_of_date),
            display_name: Set(user.display_name.clone()),
            gender: Set(user.gender.clone()),

            status: Set(user.status.clone().into()),
            role: Set(user.role.clone().into()),

            is_deleted: Set(user.is_deleted),

            verification_token: Set(user.verification_token.clone()),
            verification_token_expiry: Set(user.verification_token_expiry),
            email_verified_at: Set(user.email_verified_at),
            verification_resend_count: Set(user.verification_resend_count),
            last_verification_resend_at: Set(user.last_verification_resend_at),

            failed_login_attempts: Set(user.failed_login_attempts),
            last_failed_login_at: Set(user.last_failed_login_at),
            account_locked_until: Set(user.account_locked_until),
            last_login_at: Set(user.last_login_at),

            ..Default::default()
        }
    }

    pub fn model_to_domain(model: user_orm::Model) -> User {
        User {
            id: model.id,
            email: model.email,
            username: model.username,
            password_hash: model.password_hash,

            first_name: model.first_name,
            last_name: model.last_name,
            avatar: model.avatar,
            phone_number: model.phone_number,
            birth_of_date: model.birth_of_date,
            display_name: model.display_name,
            gender: model.gender,

            status: model.status.into(),
            role: model.role.into(),
            is_deleted: model.is_deleted,

            email_verified_at: model.email_verified_at,
            verification_token: model.verification_token,
            verification_token_expiry: model.verification_token_expiry,
            verification_resend_count: model.verification_resend_count,
            last_verification_resend_at: model.last_verification_resend_at,

            failed_login_attempts: model.failed_login_attempts,
            last_failed_login_at: model.last_failed_login_at,
            account_locked_until: model.account_locked_until,
            last_login_at: model.last_login_at,
        }
    }

    pub fn domain_to_active_model_update(user: &User) -> user_orm::ActiveModel {
        let mut active = user_orm::ActiveModel {
            id: Set(user.id),
            ..Default::default()
        };

        // identity
        active.email = Set(user.email.clone());
        active.username = Set(user.username.clone());

        // auth
        active.password_hash = Set(user.password_hash.clone());

        // profile
        active.first_name = Set(user.first_name.clone());
        active.last_name = Set(user.last_name.clone());
        active.avatar = Set(user.avatar.clone());
        active.phone_number = Set(user.phone_number.clone());
        active.birth_of_date = Set(user.birth_of_date);
        active.display_name = Set(user.display_name.clone());
        active.gender = Set(user.gender.clone());

        // state
        active.status = Set(user.status.clone().into());
        active.role = Set(user.role.clone().into());
        active.is_deleted = Set(user.is_deleted);

        // verification
        active.email_verified_at = Set(user.email_verified_at);
        active.verification_token = Set(user.verification_token.clone());
        active.verification_token_expiry = Set(user.verification_token_expiry);
        active.verification_resend_count = Set(user.verification_resend_count);
        active.last_verification_resend_at = Set(user.last_verification_resend_at);

        // login security
        active.failed_login_attempts = Set(user.failed_login_attempts);
        active.last_failed_login_at = Set(user.last_failed_login_at);
        active.account_locked_until = Set(user.account_locked_until);
        active.last_login_at = Set(user.last_login_at);

        active
    }
}
