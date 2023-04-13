use axum::{extract::State, Json};
use serde::Serialize;
use std::sync::Arc;

pub struct SpytterState {
    spyyts: Vec<Spyyt>,
}

impl SpytterState {
    pub const fn new() -> Self {
        Self { spyyts: Vec::new() }
    }
}

#[derive(Clone, Serialize)]
pub struct Spyyt {
    text: String,
}

#[allow(clippy::unused_async)]
pub async fn spyyts(
    State(state): State<Arc<SpytterState>>,
) -> Json<Vec<Spyyt>> {
    Json(state.spyyts.clone())
}
