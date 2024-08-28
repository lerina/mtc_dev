#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{sea_query::Order, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::contacts::{ActiveModel, Column, Entity, Model},
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

/*
impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.phone = Set(self.phone.clone());
        item.email = Set(self.email.clone());
    }
}
*/

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, 1).await?;
    /*    Entity::find()
        .order_by(Column::Id, Order::Desc)
        .all(&ctx.db)
        .await?;
    */
    views::frontend::index(&v, &item)
}

pub fn routes() -> Routes {
    Routes::new()
        //.prefix("")
        .add("/", get(index))
}
