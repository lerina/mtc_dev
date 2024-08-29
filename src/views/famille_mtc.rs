use loco_rs::prelude::*;
use loco_rs::{
    controller::views::pagination::{Pager, PagerMeta},
    prelude::model::query::{PageResponse, PaginationQuery},
};
use crate::models::_entities::{famille_mtcs, letypes, marques, familles, depots, centres, marque_oems};

use serde::{Deserialize, Serialize};
use sea_orm::prelude::Decimal;


#[derive(Debug, Deserialize, Serialize)]
pub struct ListResponse {
    id: i32,
    code: Option<Decimal>,
    designation: Option<String>,
    pix: Option<String>,
    letype_id: i32,
    marque_id: i32,
    prix_ttc: Option<Decimal>,
    famille_id: i32,
    depot_id: i32,
    centre_id: i32,
    reference: Option<String>,
    oem: Option<String>,
    marque_oem_id: i32,
    keywords: Option<String>,
}

impl From<famille_mtcs::Model> for ListResponse {
    fn from(item: famille_mtcs::Model) -> Self {
        Self {
            id: item.id,
            code: item.code.clone(),
            designation: item.designation.clone(),
            pix: item.pix.clone(),
            letype_id: item.letype_id,
            marque_id: item.marque_id,
            prix_ttc: item.prix_ttc.clone(),
            famille_id: item.famille_id,
            depot_id: item.depot_id,
            centre_id: item.centre_id,
            reference: item.reference.clone(),
            oem: item.oem.clone(),
            marque_oem_id: item.marque_oem_id,
            keywords: item.keywords.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationResponse {}

impl PaginationResponse {
    #[must_use]
    pub fn response(
        data: PageResponse<famille_mtcs::Model>,
        pagination_query: &PaginationQuery,
    ) -> Pager<Vec<ListResponse>> {
        Pager {
            results: data
                .page
                .into_iter()
                .map(ListResponse::from)
                .collect::<Vec<ListResponse>>(),
            info: PagerMeta {
                page: pagination_query.page,
                page_size: pagination_query.page_size,
                total_pages: data.total_pages,
            },
        }
    }
}

/// Render a list view of famille_mtcs.
///
/// # example usage
/// 
/// http://localhost:5150/admin/famille_mtcs?page=3
///
pub fn list(v: &impl ViewRenderer, 
        paginated_famille_mtcs: PageResponse<famille_mtcs::Model>, 
        pagination_query: query::PaginationQuery,) -> Result<Response> {

    let item = PaginationResponse::response(
            paginated_famille_mtcs,
            &pagination_query,
        );

    format::render().view(v, "admin/famille_mtc/list.html", 
            serde_json::json!( {"items": item, 
                                "data": item.results, 
                                "info": item.info}))
}
/*
pub fn list(v: &impl ViewRenderer, items: &Vec<famille_mtcs::Model>) -> Result<Response> {
    format::render().view(v, "famille_mtc/list.html", serde_json::json!({"items": items}))
}
*/

/// Render a single famille_mtc view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &famille_mtcs::Model) -> Result<Response> {
    format::render().view(v, "admin/famille_mtc/show.html", serde_json::json!({"item": item}))
}

/// Render a famille_mtc create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer, letypes: &Vec<letypes::Model>,
            marques: &Vec<marques::Model>, familles: &Vec<familles::Model>, depots: &Vec<depots::Model>, 
             centres: &Vec<centres::Model>,  marque_oems: &Vec<marque_oems::Model>,) -> Result<Response> {
    format::render().view(v, "admin/famille_mtc/create.html", serde_json::json!({"letypes": letypes,
                             "marques": marques,
                             "familles": familles,
                             "depots": depots,
                             "centres": centres,
                             "marque_oems": marque_oems,}))
}

/// Render a famille_mtc edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &famille_mtcs::Model, letypes: &Vec<letypes::Model>,
            marques: &Vec<marques::Model>, familles: &Vec<familles::Model>, depots: &Vec<depots::Model>, 
             centres: &Vec<centres::Model>,  marque_oems: &Vec<marque_oems::Model>,) -> Result<Response> {
    
    format::render().view(v, "admin/famille_mtc/edit.html", 
        serde_json::json!(  {"item": item, 
                             "letypes": letypes,
                             "marques": marques,
                             "familles": familles,
                             "depots": depots,
                             "centres": centres,
                             "marque_oems": marque_oems,
                            }))
}
