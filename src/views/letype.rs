use loco_rs::prelude::*;

use crate::models::_entities::letypes;

/// Render a list view of letypes.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<letypes::Model>) -> Result<Response> {
    format::render().view(v, "admin/letype/list.html", serde_json::json!({"items": items}))
}

/// Render a single letype view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &letypes::Model) -> Result<Response> {
    format::render().view(v, "admin/letype/show.html", serde_json::json!({"item": item}))
}

/// Render a letype create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "admin/letype/create.html", serde_json::json!({}))
}

/// Render a letype edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &letypes::Model) -> Result<Response> {
    format::render().view(v, "admin/letype/edit.html", serde_json::json!({"item": item}))
}
