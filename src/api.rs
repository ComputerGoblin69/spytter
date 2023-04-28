use axum::{
    extract::{ws::Message, State, WebSocketUpgrade},
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{
    broadcast::{self, Sender},
    RwLock,
};

const UWU_PROBABILITY: f32 = 0.05;

pub struct SpytterState {
    spyyts: RwLock<Vec<Spyyt>>,
    tx: Sender<Spyyt>,
}

impl SpytterState {
    pub fn new() -> Arc<Self> {
        let (tx, mut rx) = broadcast::channel(16);

        let state = Arc::new(Self {
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
            tx,
        });

        {
            let state = state.clone();
            tokio::spawn(async move {
                while let Ok(spyyt) = rx.recv().await {
                    state.spyyts.write().await.push(spyyt);
                }
            });
        }

        state
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Spyyt {
    text: String,
}

pub async fn spyyts(
    ws: WebSocketUpgrade,
    State(state): State<Arc<SpytterState>>,
) -> Response {
    let mut rx = state.tx.subscribe();

    ws.on_upgrade(|mut socket| async move {
        let spyyts = state.spyyts.read().await;
        socket
            .send(Message::Text(serde_json::to_string(&*spyyts).unwrap()))
            .await
            .unwrap();

        tokio::spawn(async move {
            while let Ok(spyyt) = rx.recv().await {
                if socket
                    .send(Message::Text(
                        serde_json::to_string(&[spyyt]).unwrap(),
                    ))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });
    })
}

#[allow(clippy::unused_async)]
pub async fn post_spyyt(
    State(state): State<Arc<SpytterState>>,
    Json(mut spyyt): Json<Spyyt>,
) {
    // TODO: Return some sort of error when the message is too long
    if spyyt.text.len() <= 281 {
        if fastrand::f32() < UWU_PROBABILITY {
            spyyt.text = uwuifier::uwuify_str_sse(&spyyt.text);
        }

        state.tx.send(spyyt).ok();
    }
}
