#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20240827_080704_letypes;
mod m20240827_081408_marques;
mod m20240827_082242_familles;
mod m20240827_082918_depots;
mod m20240827_083758_centres;
mod m20240827_084552_marque_oems;
mod m20240827_085421_contacts;
mod m20240827_090253_famille_mtcs;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20240827_080704_letypes::Migration),
            Box::new(m20240827_081408_marques::Migration),
            Box::new(m20240827_082242_familles::Migration),
            Box::new(m20240827_082918_depots::Migration),
            Box::new(m20240827_083758_centres::Migration),
            Box::new(m20240827_084552_marque_oems::Migration),
            Box::new(m20240827_085421_contacts::Migration),
            Box::new(m20240827_090253_famille_mtcs::Migration),
        ]
    }
}