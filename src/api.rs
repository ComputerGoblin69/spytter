use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub struct SpytterState {
    spyyts: RwLock<Vec<Spyyt>>,
}

impl SpytterState {
    pub fn new() -> Self {
        Self {
            spyyts: RwLock::new(vec![
                Spyyt {
                    text: "Example spyyt 1".to_owned(),
                },
                Spyyt {
                    text: "Example spyyt 2".to_owned(),
                },
                Spyyt {
                    text: "Example spyyt 3".to_owned(),
                },
            ]),
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
    // TODO: Return some sort of error when the message is too long
    if spyyt.text.len() <= 281 {
        state.spyyts.write().unwrap().push(spyyt);
    }
}
