use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlertConfig::Table)
                    .if_not_exists()
                    .col(pk_auto(AlertConfig::Id))
                    .col(string(AlertConfig::UserId))
                    .col(string(AlertConfig::IntegrationContacts))
                    .col(string(AlertConfig::NearbyUserThreshold))
                    .col(string(AlertConfig::MovementThreshold))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AlertConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AlertConfig {
    Table,
    Id,
    UserId,
    IntegrationContacts,
    NearbyUserThreshold,
    MovementThreshold,
}
