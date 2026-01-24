use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use crate::domain::user::rules::{
    AccountMustBeActive, AccountMustNotBeLocked, EmailMustBeValid,
    FailedLoginLimitMustNotBeExceeded, FullNameMustBeValid, PasswordMustMeetRequirements,
    PhoneMustBeValid, UserMustBeAtLeastAge, UserMustNotBeAlreadyVerified,
    VerificationResendLimitMustNotBeExceeded, VerificationTokenMustNotBeExpired,
};
use chrono::{NaiveDate, NaiveDateTime};
use rand::{rngs::OsRng, RngCore};

#[derive(Debug, Clone)]
pub struct RegisterUserProps {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone_number: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    pub gender: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateUserProps {
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub display_name: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateUserProps {
    pub avatar: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub display_name: Option<String>,
    pub gender: Option<String>,
}

impl RegisterUserProps {
    pub fn validate(&self, today: NaiveDate) -> Result<(), DomainError> {
        // Rule: Email must be valid
        EmailMustBeValid {
            email: self.email.clone(),
        }
        .check_broken()?;
        // Rule: Password must meet requirements
        PasswordMustMeetRequirements {
            password: self.password.clone(),
        }
        .check_broken()?;
        // Rule: Full name must be valid
        FullNameMustBeValid {
            full_name: self.full_name.clone(),
        }
        .check_broken()?;
        // Rule: Phone must be valid if provided
        if let Some(ref phone) = self.phone_number {
            PhoneMustBeValid {
                phone: phone.clone(),
            }
            .check_broken()?;
        }
        // Rule: User must be at least 13 years old
        UserMustBeAtLeastAge {
            date_of_birth: self.birth_of_date,
            minimum_age: 13,
            today,
        }
        .check_broken()?;

        Ok(())
    }
}

impl CreateUserProps {
    pub fn validate(&self, today: NaiveDate) -> Result<(), DomainError> {
        // required
        EmailMustBeValid {
            email: self.email.clone(),
        }
        .check_broken()?;

        if let Some(ref phone) = self.phone_number {
            PhoneMustBeValid {
                phone: phone.clone(),
            }
            .check_broken()?;
        }

        UserMustBeAtLeastAge {
            date_of_birth: self.birth_of_date,
            minimum_age: 13,
            today,
        }
        .check_broken()?;

        Ok(())
    }
}
impl UpdateUserProps {
    pub fn validate(&self, today: NaiveDate) -> Result<(), DomainError> {
        if let Some(ref email) = self.email {
            EmailMustBeValid {
                email: email.clone(),
            }
            .check_broken()?;
        }

        if let Some(ref phone) = self.phone_number {
            PhoneMustBeValid {
                phone: phone.clone(),
            }
            .check_broken()?;
        }

        if self.birth_of_date.is_some() {
            UserMustBeAtLeastAge {
                date_of_birth: self.birth_of_date,
                minimum_age: 13,
                today,
            }
            .check_broken()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    //pub uuid: Uuid,
    pub email: String,
    pub username: String,

    // auth
    pub password_hash: Option<String>,

    // profile
    pub first_name: String,
    pub last_name: String,
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    pub display_name: Option<String>,
    pub gender: Option<String>,

    // state
    pub status: UserStatus,
    pub role: UserRole,
    pub is_deleted: bool,

    // verification
    pub email_verified_at: Option<NaiveDateTime>,
    pub verification_token: Option<String>,
    pub verification_token_expiry: Option<NaiveDateTime>,
    pub verification_resend_count: i32,
    pub last_verification_resend_at: Option<NaiveDateTime>,

    // login security
    pub failed_login_attempts: i32,
    pub last_failed_login_at: Option<NaiveDateTime>,
    pub account_locked_until: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Pending,
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Customer,
    Admin,
}

// Domain Rules - Create and validate Models
// Validates all business rules before creating the model
impl User {
    // Rule: Create a new user for registration
    pub fn new_for_registration(
        props: RegisterUserProps,
        today: NaiveDate,
    ) -> Result<Self, DomainError> {
        // Check all rules
        props.validate(today)?;

        // Parse full_name into first_name and last_name
        /*let name_parts: Vec<&str> = props.full_name.trim().split_whitespace().collect();
        let (first_name, last_name) = match name_parts.len() {
            0 => (None, None),
            1 => (Some(name_parts[0].to_string()), Some("".to_string())),
            _ => (
                Some(name_parts[0].to_string()),
                Some(name_parts[1..].join(" ")),
            ),
        };*/
        let (first_name, last_name) = split_full_name(&props.full_name);

        // Generate username from email (part before @)
        let username = generate_username_from_email(&props.email)?;

        // Create and return the user
        Ok(Self {
            id: 0,
            email: props.email,
            username,
            password_hash: Some(props.password),

            first_name,
            last_name,
            avatar: None,
            phone_number: props.phone_number,
            birth_of_date: props.birth_of_date,
            display_name: Some(props.full_name),
            gender: props.gender,

            status: UserStatus::Pending,
            role: UserRole::Customer,
            is_deleted: false,

            email_verified_at: None,
            verification_token: None,
            verification_token_expiry: None,
            verification_resend_count: 0,
            last_verification_resend_at: None,

            failed_login_attempts: 0,
            last_failed_login_at: None,
            account_locked_until: None,
            last_login_at: None,
        })
    }

    // Rule: Create a new user model with validation
    pub fn create_new_user(props: &CreateUserProps, today: NaiveDate) -> Result<Self, DomainError> {
        // Validate domain rules
        props.validate(today)?;
        let username = generate_username_from_email(&props.email)?;

        // Create and return the user model
        Ok(Self {
            id: 0,
            email: props.email.clone(),
            username,

            password_hash: None,

            // profile
            first_name: props.first_name.clone(),
            last_name: props.last_name.clone(),

            avatar: props.avatar.clone(),
            phone_number: props.phone_number.clone(),
            birth_of_date: props.birth_of_date,
            display_name: props.display_name.clone(),
            gender: props.gender.clone(),

            // state
            status: UserStatus::Pending,
            role: UserRole::Customer,
            is_deleted: false,

            // verification
            email_verified_at: None,
            verification_token: None,
            verification_token_expiry: None,
            verification_resend_count: 0,
            last_verification_resend_at: None,

            // login security
            failed_login_attempts: 0,
            last_failed_login_at: None,
            account_locked_until: None,
            last_login_at: None,
        })
    }

    // Rule: Update user model with validation
    pub fn update_from(
        &mut self,
        props: &UpdateUserProps,
        now: NaiveDate,
    ) -> Result<(), DomainError> {
        props.validate(now)?;

        if let Some(ref avatar) = props.avatar {
            self.avatar = Some(avatar.clone());
        }
        if let Some(ref first_name) = props.first_name {
            self.first_name = first_name.clone();
        }
        if let Some(ref last_name) = props.last_name {
            self.last_name = last_name.clone();
        }
        if let Some(ref email) = props.email {
            self.email = email.clone();
            self.email_verified_at = None;
        }
        if let Some(bod) = props.birth_of_date {
            self.birth_of_date = Some(bod);
        }
        if let Some(ref phone) = props.phone_number {
            self.phone_number = Some(phone.clone());
        }
        if let Some(ref display_name) = props.display_name {
            self.display_name = Some(display_name.clone());
        }
        if let Some(ref gender) = props.gender {
            self.gender = Some(gender.clone());
        }

        Ok(())
    }

    // Rule: Verify user email
    // Validates business rules and transitions user from pending to active
    pub fn verify_email(&mut self, now: NaiveDateTime) -> Result<(), DomainError> {
        // Rule: User must not be already verified
        UserMustNotBeAlreadyVerified {
            email_verified_at: self.email_verified_at,
        }
        .check_broken()?;
        // Rule: Verification token must not be expired
        VerificationTokenMustNotBeExpired {
            token_expiry: self.verification_token_expiry,
            now,
        }
        .check_broken()?;
        // Update user status and verification fields
        self.status = UserStatus::Active;
        self.email_verified_at = Some(now);
        self.verification_token = None;
        self.verification_token_expiry = None;

        Ok(())
    }

    // Business Rule: Prepare for verification email resend
    pub fn prepare_resend_verification(
        &mut self,
        new_token: String,
        new_expiry: NaiveDateTime,
        now: NaiveDateTime,
    ) -> Result<(), DomainError> {
        // Rule: User must not be already verified
        UserMustNotBeAlreadyVerified {
            email_verified_at: self.email_verified_at,
        }
        .check_broken()?;

        // Reset counter if > 1 hour has passed since last resend
        if let Some(last_resend) = self.last_verification_resend_at {
            let one_hour_ago = now - chrono::Duration::hours(1);
            if last_resend <= one_hour_ago {
                self.verification_resend_count = 0;
            }
        }
        // Rule: Resend limit must not be exceeded (max 3 per hour)
        VerificationResendLimitMustNotBeExceeded {
            resend_count: self.verification_resend_count,
            last_resend_at: self.last_verification_resend_at,
            max_resends_per_hour: 3,
            now,
        }
        .check_broken()?;
        // Update verification token and tracking fields
        self.verification_token = Some(new_token);
        self.verification_token_expiry = Some(new_expiry);
        self.verification_resend_count += 1;
        self.last_verification_resend_at = Some(now);

        Ok(())
    }

    // Rule: Validate login attempt
    // Checks account status and lock status before password verification
    pub fn validate_login_attempt(&self, now: NaiveDateTime) -> Result<(), DomainError> {
        // Rule: Account must not be locked
        AccountMustNotBeLocked {
            account_locked_until: self.account_locked_until,
            now,
        }
        .check_broken()?;
        // Rule: Account must be active
        AccountMustBeActive {
            status: self.status.clone(),
        }
        .check_broken()?;
        // Rule: Failed login limit must not be exceeded
        FailedLoginLimitMustNotBeExceeded {
            failed_attempts: self.failed_login_attempts,
            last_failed_login_at: self.last_failed_login_at,
            max_attempts: 5,
            lockout_window_minutes: 15,
            now,
        }
        .check_broken()?;

        Ok(())
    }

    // Rule: Handle failed login attempt
    // Increments failed login counter and locks account if threshold exceeded
    pub fn handle_failed_login(&mut self, now: NaiveDateTime) {
        // Reset counter if > 15 mins passed since last failed attempt
        if let Some(last_failed) = self.last_failed_login_at {
            let fifteen_minutes_ago = now - chrono::Duration::minutes(15);
            if last_failed <= fifteen_minutes_ago {
                self.failed_login_attempts = 0;
            }
        }
        // Increment failed login counter
        self.failed_login_attempts += 1;
        self.last_failed_login_at = Some(now);

        // Lock account for 30 minutes if 5 or more failed attempts
        if self.failed_login_attempts >= 5 {
            self.account_locked_until = Some(now + chrono::Duration::minutes(30));
        }
    }

    // Rule: Handle successful login
    // Resets failed login counter and updates last login timestamp
    pub fn handle_successful_login(&mut self, now: NaiveDateTime) {
        // Reset failed login tracking
        self.failed_login_attempts = 0;
        self.last_failed_login_at = None;
        self.account_locked_until = None;
        // Update last login timestamp
        self.last_login_at = Some(now);
    }
}

fn split_full_name(full_name: &str) -> (String, String) {
    let mut iter = full_name.trim().split_whitespace();

    let first_name = iter.next().unwrap_or("").to_string();
    let last_name = iter.collect::<Vec<&str>>().join(" ");

    (first_name, last_name)
}
fn normalize_username(input: &str) -> String {
    let mut out = String::with_capacity(input.len());

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if ch == '.' || ch == '-' || ch == '+' {
            out.push('_');
        }
    }

    // remove duplicated underscores
    while out.contains("__") {
        out = out.replace("__", "_");
    }

    out = out.trim_matches('_').to_string();

    if out.is_empty() {
        out = "user".to_string();
    }

    out
}

fn random_base36(len: usize) -> String {
    const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

    let mut bytes = vec![0u8; len];
    OsRng.fill_bytes(&mut bytes);

    bytes
        .into_iter()
        .map(|b| ALPHABET[(b as usize) % ALPHABET.len()] as char)
        .collect()
}

fn generate_username_from_email(email: &str) -> Result<String, DomainError> {
    let (local_part, _) = email.split_once('@').ok_or_else(|| {
        UserDomainError::Validation {
            field: "email",
            message: "Invalid email format".to_string(),
        }
        // UserDomainError → DomainError
    })?;

    // base username
    let mut base = normalize_username(local_part);

    // username total length <= 30
    // format: base + "_" + suffix(8) => base max 21
    const SUFFIX_LEN: usize = 8;
    const MAX_USERNAME_LEN: usize = 30;
    let max_base_len = MAX_USERNAME_LEN - 1 - SUFFIX_LEN;

    if base.len() > max_base_len {
        base.truncate(max_base_len);
    }

    let suffix = random_base36(SUFFIX_LEN);

    Ok(format!("{base}_{suffix}"))
}
