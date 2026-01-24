use crate::core::audit::audit_fields::AuditFields;
use crate::core::context::request_context::RequestContext;
use sea_orm::DbErr;

pub fn apply_audit_fields<M>(
    mut model: M,
    ctx: Option<&dyn RequestContext>,
    insert: bool,
) -> Result<M, DbErr>
where
    M: AuditFields,
{
    let now = chrono::Utc::now().naive_utc();

    let user_id = ctx.and_then(|c| c.user_id());

    if insert {
        model.set_created_at(now);
        model.set_created_by(user_id);
    }

    model.set_updated_at(now);
    model.set_updated_by(user_id);

    Ok(model)
}
