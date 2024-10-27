use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Integration::Table)
                    .if_not_exists()
                    .col(pk_auto(Integration::Id))
                    .col(string(Integration::Name))
                    .col(string(Integration::Type))
                    .col(string(Integration::ContactInfo))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Integration::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Integration {
    Table,
    Id,
    Name,
    Type,
    ContactInfo,
}
