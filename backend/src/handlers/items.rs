use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domain::items::{Account, Item, ItemType},
};

// ----------------------------------------------------------
// DTO definitions
// ----------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ListItemTypesQuery {
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub date: NaiveDate,
    pub amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub item_type_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub entries: Vec<CreateEntryRequest>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntryRequest {
    pub date: NaiveDate,
    pub amount: i64,
}

#[derive(Deserialize)]
pub struct UpdateEntryDTO {
    pub date: NaiveDate,
    pub amount: i64,
}

#[derive(Deserialize)]
pub struct UpdateEntriesRequest {
    pub entries: Vec<UpdateEntryDTO>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub assignee_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub entries: Option<Vec<UpdateEntryRequest>>,
}

// ----------------------------------------------------------
// Handlers
// ----------------------------------------------------------

pub async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Account>>, (StatusCode, String)> {
    let accounts = state.item_repository.list_accounts().await.map_err(|e| {
        tracing::error!("Failed to list accounts: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(accounts))
}

pub async fn list_item_types(
    Query(query): Query<ListItemTypesQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemType>>, (StatusCode, String)> {
    let item_types = state
        .item_repository
        .list_item_types(query.account_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list item types: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(item_types))
}

pub async fn create_item(
    Path((_project_id, job_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<Item>), (StatusCode, String)> {
    let entries_data: Vec<(NaiveDate, i64)> = payload
        .entries
        .into_iter()
        .map(|e| (e.date, e.amount))
        .collect();

    let item = state
        .item_repository
        .create(
            job_id,
            payload.item_type_id,
            payload.assignee_id,
            payload.name,
            payload.description.unwrap_or_default(),
            entries_data,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create item: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn list_items(
    Path((_project_id, job_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Item>>, (StatusCode, String)> {
    let items = state
        .item_repository
        .find_by_job_id(job_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list items: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(items))
}

pub async fn update_item(
    Path((_project_id, job_id, item_id)): Path<(Uuid, Uuid, Uuid)>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<(StatusCode, Json<Item>), (StatusCode, String)> {
    let entries = payload.entries.map(|es| {
        es.into_iter()
            .map(|e| (e.date, e.amount))
            .collect::<Vec<(NaiveDate, i64)>>()
    });

    let item = state
        .item_repository
        .update(
            item_id,
            job_id,
            payload.assignee_id,
            payload.name,
            payload.description,
            entries,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to update item: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn update_entries(
    Path((_project_id, _job_id, item_id)): Path<(Uuid, Uuid, Uuid)>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateEntriesRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let entries: Vec<(NaiveDate, i64)> = payload
        .entries
        .into_iter()
        .map(|e| (e.date, e.amount))
        .collect();

    state
        .item_repository
        .update_entries(item_id, entries)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update entries: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_item(
    Path((_project_id, job_id, item_id)): Path<(Uuid, Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let count = state
        .item_repository
        .delete(item_id, job_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete item: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if count == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            "Project or Job or Item not found".to_string(),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn export_items_csv(
    Path((_project_id, job_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let items = state
        .item_repository
        .find_by_job_id(job_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut wtr = csv::Writer::from_writer(vec![]);

    // ヘッダ行の作成
    let start_date = chrono::NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
    let mut months = Vec::new();
    for i in 0..12 {
        let d = start_date
            .checked_add_months(chrono::Months::new(i))
            .unwrap();
        months.push(d)
    }

    let mut headers = vec![
        "item_id".to_string(),
        "name".to_string(),
        "item_type_id".to_string(),
    ];
    for m in &months {
        headers.push(m.format("%Y-%m-%d").to_string());
    }
    wtr.write_record(&headers)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // データ行の書き込み
    for item in items {
        let entry_map: HashMap<chrono::NaiveDate, i64> =
            item.entries.iter().map(|e| (e.date, e.amount)).collect();

        let mut row = vec![
            item.id.to_string(),
            item.name.clone(),
            item.item_type_id.to_string(),
        ];

        for m in &months {
            let amount = entry_map.get(m).unwrap_or(&0);
            row.push(amount.to_string());
        }

        wtr.write_record(&row)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    // レスポンスの生成
    let data = wtr
        .into_inner()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let filename = format!("budget_export_{}", job_id);

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/csv; charset=utf-8"),
    );

    let content_disposition_value = format!("attachment; filename=\"{}\"", filename);
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&content_disposition_value)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    );

    Ok((headers, data))
}
