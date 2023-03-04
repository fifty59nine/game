use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use dungeon_sdk::types::*;

use super::Game;

pub const PROTO_ID: u64 = 1;

pub struct Server {
    host: String,
}

impl Server {
    pub fn new(host: String) -> Self {
        Self { host }
    }

    pub async fn run_server(&self, game: Arc<Mutex<Game>>) -> Result<Infallible, std::io::Error> {
        let listener = TcpListener::bind(&self.host).await?;

        println!(
            "DungeonServer v{} has been successfully started on {}",
            env!("CARGO_PKG_VERSION"),
            &self.host,
        );

        loop {
            let (mut socket, _) = listener.accept().await?;
            let game_clone = Arc::clone(&game);

            tokio::spawn(async move {
                let mut buf = vec![];

                loop {
                    let mut b = vec![0; 64];
                    match socket.read(&mut b).await {
                        Ok(n) if n == 0 => {
                            // socket has been closed by the client
                            return;
                        }
                        Ok(n) => {
                            buf.extend(&b[0..n]);
                            if n < 64 {
                                // this was the last chunk of data from the client
                                break;
                            }
                        }
                        Err(e) => {
                            // an error occurred while reading from the socket
                            println!("Error: {}", e);
                            return;
                        }
                    }
                }
                let request_message = String::from_utf8(buf).unwrap().trim().to_owned();
                println!(" < Accepted: {}", &request_message);
                let ans = Self::handle_request(
                    &request_message,
                    game_clone,
                    &socket.peer_addr().unwrap().to_string(),
                );
                println!(" > Answer: {}", &ans);
                if let Err(_) = socket.write_all(ans.as_bytes()).await {
                    return;
                }
            });
        }
    }

    fn handle_request(request: &str, game: Arc<Mutex<Game>>, addr: &str) -> String {
        if let Ok(req) = serde_json::from_str::<Request>(request) {
            if req.proto_id != PROTO_ID {
                let resp = Response {
                    successful: false,
                    message: String::from("Помилка! Неправильний ID протоколу!"),
                };
                return serde_json::to_string(&resp).unwrap();
            }
            let resp = match req.body {
                Method::Ping => Ok(String::from("Ping")),
                Method::Connect(data) => {
                    let mut g_lock = game.lock().unwrap();
                    g_lock.add_player(data.clone(), addr.to_string())
                }
                Method::GetStats => {
                    let game_lock = game.lock().unwrap();
                    match game_lock.get_player(req.user) {
                        Some(p) => Ok(serde_json::to_string(p).unwrap()),
                        None => Err(format!("Гравця не знайдено!")),
                    }
                }
            };

            serde_json::to_string(&Response {
                successful: resp.is_ok(),
                message: resp.unwrap_or_else(|err| err),
            })
            .unwrap()
        } else {
            serde_json::to_string(&Response {
                successful: false,
                message: format!("Request format error({})", addr),
            })
            .unwrap()
        }
    }
}
