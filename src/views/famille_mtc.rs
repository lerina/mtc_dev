use loco_rs::prelude::*;

use crate::models::_entities::famille_mtcs;

/// Render a list view of famille_mtcs.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<famille_mtcs::Model>) -> Result<Response> {
    format::render().view(v, "famille_mtc/list.html", serde_json::json!({"items": items}))
}

/// Render a single famille_mtc view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &famille_mtcs::Model) -> Result<Response> {
    format::render().view(v, "famille_mtc/show.html", serde_json::json!({"item": item}))
}

/// Render a famille_mtc create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "famille_mtc/create.html", serde_json::json!({}))
}

/// Render a famille_mtc edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &famille_mtcs::Model) -> Result<Response> {
    format::render().view(v, "famille_mtc/edit.html", serde_json::json!({"item": item}))
}
