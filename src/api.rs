use axum::{
    extract::{ws::Message, State, WebSocketUpgrade},
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::Arc,
};
use tokio::sync::{
    broadcast::{self, Sender},
    RwLock,
};

const DB_PATH: &str = "db";

pub struct SpytterState {
    spyyts: RwLock<Vec<Spyyt>>,
    tx: Sender<Spyyt>,
    db: File,
}

impl SpytterState {
    pub fn new() -> Arc<Self> {
        let (tx, mut rx) = broadcast::channel(16);

        let (spyyts, db) = File::options()
            .read(true)
            .append(true)
            .open(DB_PATH)
            .map_or_else(
                |_| (Vec::new(), File::create(DB_PATH).unwrap()),
                |mut db| {
                    (
                        BufReader::new(&mut db)
                            .lines()
                            .map(|line| Spyyt {
                                text: line.unwrap(),
                            })
                            .collect(),
                        db,
                    )
                },
            );

        let state = Arc::new(Self {
            spyyts: RwLock::new(spyyts),
            tx,
            db,
        });

        {
            let state = state.clone();
            tokio::spawn(async move {
                while let Ok(spyyt) = rx.recv().await {
                    writeln!(&state.db, "{}", spyyt.text).unwrap();
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
    Json(spyyt): Json<Spyyt>,
) {
    // TODO: Return some sort of error when the message is too long
    if !(1..=281).contains(&spyyt.text.len()) {
        return;
    }

    state.tx.send(spyyt).ok();
}
