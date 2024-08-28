use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(FamilleMtcs::Table)
                    .col(pk_auto(FamilleMtcs::Id))
                    .col(decimal_null(FamilleMtcs::Code))
                    .col(text_null(FamilleMtcs::Designation))
                    .col(string_null(FamilleMtcs::Pix))
                    .col(integer(FamilleMtcs::LetypeId))
                    .col(integer(FamilleMtcs::MarqueId))
                    .col(decimal_null(FamilleMtcs::PrixTtc))
                    .col(integer(FamilleMtcs::FamilleId))
                    .col(integer(FamilleMtcs::DepotId))
                    .col(integer(FamilleMtcs::CentreId))
                    .col(string_null(FamilleMtcs::Reference))
                    .col(string_null(FamilleMtcs::Oem))
                    .col(integer(FamilleMtcs::MarqueOemId))
                    .col(string_null(FamilleMtcs::Keywords))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-famille_mtcs-letypes")
                            .from(FamilleMtcs::Table, FamilleMtcs::LetypeId)
                            .to(Letypes::Table, Letypes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-famille_mtcs-marques")
                            .from(FamilleMtcs::Table, FamilleMtcs::MarqueId)
                            .to(Marques::Table, Marques::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-famille_mtcs-familles")
                            .from(FamilleMtcs::Table, FamilleMtcs::FamilleId)
                            .to(Familles::Table, Familles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-famille_mtcs-depots")
                            .from(FamilleMtcs::Table, FamilleMtcs::DepotId)
                            .to(Depots::Table, Depots::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-famille_mtcs-centres")
                            .from(FamilleMtcs::Table, FamilleMtcs::CentreId)
                            .to(Centres::Table, Centres::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-famille_mtcs-marque_oems")
                            .from(FamilleMtcs::Table, FamilleMtcs::MarqueOemId)
                            .to(MarqueOems::Table, MarqueOems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FamilleMtcs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FamilleMtcs {
    Table,
    Id,
    Code,
    Designation,
    Pix,
    LetypeId,
    MarqueId,
    PrixTtc,
    FamilleId,
    DepotId,
    CentreId,
    Reference,
    Oem,
    MarqueOemId,
    Keywords,
    
}


#[derive(DeriveIden)]
enum Letypes {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Marques {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Familles {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Depots {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Centres {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum MarqueOems {
    Table,
    Id,
}
