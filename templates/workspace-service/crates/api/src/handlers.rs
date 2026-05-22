use axum::{extract::{Path, State, Query}, Json};
use uuid::Uuid;
use serde::Deserialize;

use crate::error::AppError;
use {{workspace_name}}_core::{Item, Error as CoreError};

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn list_items(
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Item>>, AppError> {
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);
    // In real implementation, this would call a service
    Ok(Json(vec![]))
}

pub async fn create_item(
    Json(payload): Json<CreateItemRequest>,
) -> Result<(axum::http::StatusCode, Json<Item>), AppError> {
    let item = Item::new(payload.name, payload.description);
    Ok((axum::http::StatusCode::CREATED, Json(item)))
}

pub async fn get_item(
    Path(id): Path<Uuid>,
) -> Result<Json<Item>, AppError> {
    Err(AppError::NotFound)
}

pub async fn delete_item(
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
}