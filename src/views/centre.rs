use loco_rs::prelude::*;

use crate::models::_entities::centres;

/// Render a list view of centres.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<centres::Model>) -> Result<Response> {
    format::render().view(v, "centre/list.html", serde_json::json!({"items": items}))
}

/// Render a single centre view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &centres::Model) -> Result<Response> {
    format::render().view(v, "centre/show.html", serde_json::json!({"item": item}))
}

/// Render a centre create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "centre/create.html", serde_json::json!({}))
}

/// Render a centre edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &centres::Model) -> Result<Response> {
    format::render().view(v, "centre/edit.html", serde_json::json!({"item": item}))
}
