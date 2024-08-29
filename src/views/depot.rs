use loco_rs::prelude::*;

use crate::models::_entities::depots;

/// Render a list view of depots.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<depots::Model>) -> Result<Response> {
    format::render().view(v, "admin/depot/list.html", serde_json::json!({"items": items}))
}

/// Render a single depot view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &depots::Model) -> Result<Response> {
    format::render().view(v, "admin/depot/show.html", serde_json::json!({"item": item}))
}

/// Render a depot create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "admin/depot/create.html", serde_json::json!({}))
}

/// Render a depot edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &depots::Model) -> Result<Response> {
    format::render().view(v, "admin/depot/edit.html", serde_json::json!({"item": item}))
}
