use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(pk_uuid(Todo::Id))
                    .col(string(Todo::Title))
                    .col(date_null(Todo::DueDateWholeDay))
                    .col(date_time_null(Todo::DueDatePeriodStart))
                    .col(integer_null(Todo::DueDatePeriodDuration))
                    .col(string_null(Todo::ContentMarkdown))
                    .col(string_null(Todo::ContentPlainText))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.truncate_table(sea_query::Table::truncate()).await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Title,
    DueDateWholeDay,
    DueDatePeriodStart,
    DueDatePeriodDuration,
    Status,
    ContentMarkdown,
    ContentPlainText,
}
