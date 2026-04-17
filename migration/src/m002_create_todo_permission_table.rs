use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TodoPermission::Table)
                    .if_not_exists()
                    .col(uuid(TodoPermission::TodoId))
                    .col(uuid(TodoPermission::UserId))
                    .col(small_integer(TodoPermission::Role))
                    .col(date_time(TodoPermission::CreatedAt))
                    .col(date_time(TodoPermission::UpdatedAt))
                    .primary_key(
                        Index::create()
                            .col(TodoPermission::TodoId)
                            .col(TodoPermission::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoPermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TodoPermission {
    Table,
    TodoId,
    UserId,
    Role,
    CreatedAt,
    UpdatedAt,
}
