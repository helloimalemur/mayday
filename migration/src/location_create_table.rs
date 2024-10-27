use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Location::Table)
                    .if_not_exists()
                    .col(pk_auto(Location::Id))
                    .col(string(Location::UserId))
                    .col(string(Location::Latitude))
                    .col(string(Location::Longitude))
                    .col(string(Location::Timestamp))
                    .col(string(Location::IsMoving))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Location::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Id,
    UserId,
    Latitude,
    Longitude,
    Timestamp,
    IsMoving,
}
