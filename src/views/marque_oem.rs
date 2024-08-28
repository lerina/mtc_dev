use loco_rs::prelude::*;

use crate::models::_entities::marque_oems;

/// Render a list view of marque_oems.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<marque_oems::Model>) -> Result<Response> {
    format::render().view(v, "marque_oem/list.html", serde_json::json!({"items": items}))
}

/// Render a single marque_oem view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &marque_oems::Model) -> Result<Response> {
    format::render().view(v, "marque_oem/show.html", serde_json::json!({"item": item}))
}

/// Render a marque_oem create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "marque_oem/create.html", serde_json::json!({}))
}

/// Render a marque_oem edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &marque_oems::Model) -> Result<Response> {
    format::render().view(v, "marque_oem/edit.html", serde_json::json!({"item": item}))
}
