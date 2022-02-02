use crate::events;
use crate::result::Result;
use axum::extract::Extension;
use axum::response::{IntoResponse, Json};
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;

pub async fn list(
    Extension(ref db): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse> {
    let events = events::Entity::find()
        .order_by_asc(events::Column::Title)
        .all(db)
        .await?;
    Ok(Json(events))
}
