#[macro_export]
macro_rules! impl_audit_for_entity {
    ($active_model:path) => {
        impl crate::core::audit::audit_fields::AuditFields for $active_model {
            fn set_created_at(&mut self, time: chrono::NaiveDateTime) {
                self.created_at = sea_orm::Set(Some(time));
            }

            fn set_updated_at(&mut self, time: chrono::NaiveDateTime) {
                self.updated_at = sea_orm::Set(Some(time));
            }

            fn set_created_by(&mut self, user_id: Option<i64>) {
                self.created_by = sea_orm::Set(user_id);
            }

            fn set_updated_by(&mut self, user_id: Option<i64>) {
                self.updated_by = sea_orm::Set(user_id);
            }
        }

        #[async_trait::async_trait]
        impl sea_orm::ActiveModelBehavior for $active_model {
            async fn before_save<C>(
                self,
                _db: &C,
                insert: bool,
            ) -> Result<Self, sea_orm::DbErr>
            where
                C: sea_orm::ConnectionTrait,
            {
                crate::infrastructure::persistence::seaorm::base_behavior::apply_audit_fields(
                    self,
                    None,
                    insert,
                )
            }
        }
    };
}
