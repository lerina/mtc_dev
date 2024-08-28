use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Letypes::Table)
                    .col(pk_auto(Letypes::Id))
                    .col(string_null(Letypes::Letype))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Letypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Letypes {
    Table,
    Id,
    Letype,
    
}


