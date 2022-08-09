use entity::*;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220702_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        return manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::FirstName)
                            .string_len(254)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::LastName)
                            .string_len(254)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Email)
                            .string_len(254)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Password)
                            .string_len(254)
                            .not_null(),
                    )
                    .col(ColumnDef::new(user::Column::Salt).string_len(32).not_null())
                    .col(
                        ColumnDef::new(user::Column::Enabled)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(user::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await;
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        return manager
            .drop_table(sea_query::Table::drop().table(user::Entity).to_owned())
            .await;
    }
}
