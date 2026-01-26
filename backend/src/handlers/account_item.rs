use axum::{Json, extract::State};

use crate::{AppState, domains::account_item::AccountItem, error::Result};

pub async fn list_account_items(State(state): State<AppState>) -> Result<Json<Vec<AccountItem>>> {
    let items = state.account_item_repository.find_all().await?;

    Ok(Json(items))
}
