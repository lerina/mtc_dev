use loco_rs::prelude::*;

use crate::models::_entities::contacts;

/// Render a list view of contacts.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<contacts::Model>) -> Result<Response> {
    format::render().view(v, "contact/list.html", serde_json::json!({"items": items}))
}

/// Render a single contact view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &contacts::Model) -> Result<Response> {
    format::render().view(v, "contact/show.html", serde_json::json!({"item": item}))
}

/// Render a contact create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "contact/create.html", serde_json::json!({}))
}

/// Render a contact edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &contacts::Model) -> Result<Response> {
    format::render().view(v, "contact/edit.html", serde_json::json!({"item": item}))
}
