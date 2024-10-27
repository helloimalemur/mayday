use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LocationData::Table)
                    .if_not_exists()
                    .col(pk_auto(LocationData::Id))
                    .col(string(LocationData::UserId))
                    .col(string(LocationData::Latitude))
                    .col(string(LocationData::Longitude))
                    .col(string(LocationData::IsMoving))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LocationData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LocationData {
    Table,
    Id,
    UserId,
    Latitude,
    Longitude,
    IsMoving,
}
