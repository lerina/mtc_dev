use loco_rs::prelude::*;

use crate::models::_entities::marques;

/// Render a list view of marques.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<marques::Model>) -> Result<Response> {
    format::render().view(v, "marque/list.html", serde_json::json!({"items": items}))
}

/// Render a single marque view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &marques::Model) -> Result<Response> {
    format::render().view(v, "marque/show.html", serde_json::json!({"item": item}))
}

/// Render a marque create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "marque/create.html", serde_json::json!({}))
}

/// Render a marque edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &marques::Model) -> Result<Response> {
    format::render().view(v, "marque/edit.html", serde_json::json!({"item": item}))
}
