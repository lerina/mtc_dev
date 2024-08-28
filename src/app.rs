use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    worker::{AppWorker, Processor},
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::{
    controllers, initializers,
    models::_entities::{notes, users, letypes, marques, familles, depots, centres, marque_oems, contacts, famille_mtcs,},
    tasks,
    workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::famille_mtc::routes())
            .add_route(controllers::contact::routes())
            .add_route(controllers::marque_oem::routes())
            .add_route(controllers::centre::routes())
            .add_route(controllers::depot::routes())
            .add_route(controllers::famille::routes())
            .add_route(controllers::marque::routes())
            .add_route(controllers::letype::routes())
            .add_route(controllers::notes::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
    }

    fn connect_workers<'a>(p: &'a mut Processor, ctx: &'a AppContext) {
        p.register(DownloadWorker::build(ctx));
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        truncate_table(db, notes::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        db::seed::<notes::ActiveModel>(db, &base.join("notes.yaml").display().to_string()).await?;
        //
        db::seed::<letypes::ActiveModel>(db, &base.join("letypes.yaml").display().to_string()).await?;
        db::seed::<marques::ActiveModel>(db, &base.join("marques.yaml").display().to_string()).await?;
        db::seed::<familles::ActiveModel>(db, &base.join("familles.yaml").display().to_string()).await?;
        db::seed::<depots::ActiveModel>(db, &base.join("depots.yaml").display().to_string()).await?;
        db::seed::<centres::ActiveModel>(db, &base.join("centres.yaml").display().to_string()).await?;
        db::seed::<marque_oems::ActiveModel>(db, &base.join("marque_oems.yaml").display().to_string()).await?;
        db::seed::<contacts::ActiveModel>(db, &base.join("contacts.yaml").display().to_string()).await?;
        db::seed::<famille_mtcs::ActiveModel>(db, &base.join("famille_mtcs.yaml").display().to_string()).await?;
        //
        Ok(())
    }
}
