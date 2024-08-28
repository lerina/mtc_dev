use loco_rs::prelude::*;

use crate::models::_entities::familles;

/// Render a list view of familles.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<familles::Model>) -> Result<Response> {
    format::render().view(v, "famille/list.html", serde_json::json!({"items": items}))
}

/// Render a single famille view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &familles::Model) -> Result<Response> {
    format::render().view(v, "famille/show.html", serde_json::json!({"item": item}))
}

/// Render a famille create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "famille/create.html", serde_json::json!({}))
}

/// Render a famille edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &familles::Model) -> Result<Response> {
    format::render().view(v, "famille/edit.html", serde_json::json!({"item": item}))
}
