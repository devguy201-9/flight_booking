#[macro_export]
macro_rules! impl_audit_for_entity {
    ($active_model:path) => {
        impl crate::infrastructure::persistence::seaorm::base_behavior::Auditable
            for $active_model
        {
            fn apply_create_audit(
                &mut self,
                ctx: &crate::core::context::request_context::RequestContext,
            ) {
                let now = chrono::Utc::now().naive_utc();

                self.created_at = sea_orm::Set(Some(now));
                self.created_by = sea_orm::Set(ctx.user_id());

                // optional but recommended
                self.updated_at = sea_orm::Set(Some(now));
                self.updated_by = sea_orm::Set(ctx.user_id());
            }

            fn apply_update_audit(
                &mut self,
                ctx: &crate::core::context::request_context::RequestContext,
            ) {
                self.updated_at = sea_orm::Set(Some(chrono::Utc::now().naive_utc()));
                self.updated_by = sea_orm::Set(ctx.user_id());
            }
        }
    };
}
