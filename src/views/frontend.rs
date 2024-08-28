use loco_rs::prelude::*;

use crate::models::_entities::contacts;

pub fn index(v: &impl ViewRenderer, items: &contacts::Model) -> Result<Response> {
    format::render().view(v, "mtc/index.html", serde_json::json!({"contact": items}))
}
