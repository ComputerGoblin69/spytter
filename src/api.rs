use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub struct SpytterState {
    spyyts: RwLock<Vec<Spyyt>>,
}

impl SpytterState {
    pub const fn new() -> Self {
        Self {
            spyyts: RwLock::new(Vec::new()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Spyyt {
    text: String,
}

#[allow(clippy::unused_async)]
pub async fn spyyts(
    State(state): State<Arc<SpytterState>>,
) -> Json<Vec<Spyyt>> {
    Json(state.spyyts.read().unwrap().clone())
}

#[allow(clippy::unused_async)]
pub async fn post_spyyt(
    State(state): State<Arc<SpytterState>>,
    Json(spyyt): Json<Spyyt>,
) {
    state.spyyts.write().unwrap().push(spyyt);
}
