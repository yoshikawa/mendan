pub use sea_orm_migration::prelude::*;

mod m20220702_000001_create_user;
mod m20220702_000002_create_reservation;
mod m20220702_000003_create_pre_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220702_000001_create_user::Migration)];
        vec![Box::new(m20220702_000002_create_reservation::Migration)];
        vec![Box::new(m20220702_000003_create_pre_user::Migration)]
    }
}
