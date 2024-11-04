use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(integer(User::UserId))
                    .col(string(User::Name))
                    .col(string(User::Email))
                    .col(string(User::Secret))
                    .to_owned(),
            )
            .await
    }
// https://www.sea-ql.org/SeaORM/docs/0.4.x/generate-entity/entity-structure/
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserId,
    Name,
    Email,
    Secret,
}
