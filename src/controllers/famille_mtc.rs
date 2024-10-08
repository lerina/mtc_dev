#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query};
use sea_orm::prelude::Decimal;
use sea_orm::{sea_query::Order, QueryOrder};
use serde::{Deserialize, Serialize};

use loco_rs::prelude::*;

use crate::models::_entities::{centres, depots, familles, letypes, marque_oems, marques};
use crate::{
    models::_entities::famille_mtcs::{ActiveModel, Column, Entity, Model},
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub code: Option<Decimal>,
    pub designation: Option<String>,
    pub pix: Option<String>,
    pub prix_ttc: Option<Decimal>,
    pub reference: Option<String>,
    pub oem: Option<String>,
    pub keywords: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.code = Set(self.code.clone());
        item.designation = Set(self.designation.clone());
        item.pix = Set(self.pix.clone());
        item.prix_ttc = Set(self.prix_ttc.clone());
        item.reference = Set(self.reference.clone());
        item.oem = Set(self.oem.clone());
        item.keywords = Set(self.keywords.clone());
    }
}

#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
    pub code: Option<Decimal>,
    pub designation: Option<String>,
    pub pix: Option<String>,
    pub prix_ttc: Option<Decimal>,
    pub reference: Option<String>,
    pub oem: Option<String>,
    pub keywords: Option<String>,
    #[serde(flatten)]
    pub pagination: query::PaginationQuery,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

/*
#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = Entity::find()
        .order_by(Column::Id, Order::Desc)
        .all(&ctx.db)
        .await?;
    views::famille_mtc::list(&v, &item)
}
*/

#[debug_handler]
pub async fn list(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Query(params): Query<ListQueryParams>,
) -> Result<Response> {
    let pagination_query = query::PaginationQuery {
        page_size: params.pagination.page_size,
        page: params.pagination.page,
    };
    let /*mut*/ condition = query::condition();
    let paginated_famille_mtcs = query::paginate(
        &ctx.db,
        Entity::find().order_by(Column::Id, Order::Desc),
        Some(condition.build()), //Some(query::with(params.into_query()).build()),
        &pagination_query,
    )
    .await?;

    views::famille_mtc::list(&v, paginated_famille_mtcs, pagination_query)
}

/*
//to build conditions with
// `Some(query::with(params.into_query()).build()),`
// instead of `Some(condition.build())`,
impl ListQueryParams {
    #[must_use]
    pub fn into_query(&self) -> Condition {
        let mut condition = query::condition();

        if let Some(title) = &self.title {
            condition = condition.like(Column::Title, title);
        }
        if let Some(content) = &self.content {
            condition = condition.like(Column::Content, content);
        }
      condition = Entity::find().order_by(Column::Id, Order::Desc);
        condition.build()
    }
}
*/

#[debug_handler]
pub async fn new(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let letypes = letypes::Entity::find().all(&ctx.db).await?;
    let marques = marques::Entity::find().all(&ctx.db).await?;
    let familles = familles::Entity::find().all(&ctx.db).await?;
    let depots = depots::Entity::find().all(&ctx.db).await?;
    let centres = centres::Entity::find().all(&ctx.db).await?;
    let marque_oems = marque_oems::Entity::find().all(&ctx.db).await?;

    views::famille_mtc::create(
        &v,
        &letypes,
        &marques,
        &familles,
        &depots,
        &centres,
        &marque_oems,
    )
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;

    format::json(item)
}

#[debug_handler]
pub async fn edit(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let letypes = letypes::Entity::find().all(&ctx.db).await?;
    let marques = marques::Entity::find().all(&ctx.db).await?;
    let familles = familles::Entity::find().all(&ctx.db).await?;
    let depots = depots::Entity::find().all(&ctx.db).await?;
    let centres = centres::Entity::find().all(&ctx.db).await?;
    let marque_oems = marque_oems::Entity::find().all(&ctx.db).await?;

    views::famille_mtc::edit(
        &v,
        &item,
        &letypes,
        &marques,
        &familles,
        &depots,
        &centres,
        &marque_oems,
    )
}

#[debug_handler]
pub async fn show(
    Path(id): Path<i32>,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;

    views::famille_mtc::show(&v, &item)
}

#[debug_handler]
pub async fn add(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;

    views::famille_mtc::show(&v, &item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;

    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("admin/famille_mtcs")
        .add("/", get(list))
        .add("/new", get(new))
        .add("/:id", get(show))
        .add("/:id/edit", get(edit))
        .add("/:id", post(update))
        .add("/:id", delete(remove))
        .add("/", post(add))
}
