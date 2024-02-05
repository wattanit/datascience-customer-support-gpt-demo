use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Thread {
    Table,
    Id,
    Title,
    AssistantId,
    CustomerId,
    Status,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Thread::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Thread::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Thread::Title).string().not_null())
                    .col(ColumnDef::new(Thread::AssistantId).string().not_null())
                    .col(ColumnDef::new(Thread::CustomerId).integer().not_null())
                    .col(ColumnDef::new(Thread::Status).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Thread::Table).to_owned())
            .await
    }
}
