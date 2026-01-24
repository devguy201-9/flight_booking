pub use sea_orm_migration::prelude::*;
mod m20260111_201209_create_users;
mod m20260111_201247_create_addresses;
mod m20260111_201312_create_airports;
mod m20260111_201319_create_flights;
mod m20260111_201326_create_booking;
mod m20260111_201350_create_passengers;
mod m20260111_201358_create_checkins;
mod m20260111_201427_create_boarding_passes;
mod m20260111_201209_add_email_verification_resend_tracking;
mod m20260111_201309_add_login_tracking_fields;
pub mod helpers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260111_201209_create_users::Migration),
            Box::new(m20260111_201247_create_addresses::Migration),
            Box::new(m20260111_201312_create_airports::Migration),
            Box::new(m20260111_201319_create_flights::Migration),
            Box::new(m20260111_201326_create_booking::Migration),
            Box::new(m20260111_201350_create_passengers::Migration),
            Box::new(m20260111_201358_create_checkins::Migration),
            Box::new(m20260111_201427_create_boarding_passes::Migration),
            Box::new(m20260111_201209_add_email_verification_resend_tracking::Migration),
            Box::new(m20260111_201309_add_login_tracking_fields::Migration),
        ]
    }
}
