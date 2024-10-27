use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alert::Table)
                    .if_not_exists()
                    .col(pk_auto(Alert::Id))
                    .col(string(Alert::UserId))
                    .col(string(Alert::AlertType))
                    .col(string(Alert::Timestamp))
                    .col(string(Alert::Details))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alert::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Alert {
    Table,
    Id,
    UserId,
    AlertType,
    Timestamp,
    Details,
}
