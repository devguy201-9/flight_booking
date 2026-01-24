use crate::application::auth::token_service::{TokenPair, TokenService};
use crate::application::common::use_case_error::UseCaseResult;
use uuid::Uuid;

pub fn issue_token_pair(
    token_service: &dyn TokenService,
    user_id: i64,
    session_id: Uuid,
) -> UseCaseResult<TokenPair> {
    token_service.generate_tokens(user_id, session_id)
}
