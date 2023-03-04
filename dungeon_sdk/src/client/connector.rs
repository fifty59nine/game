use crate::logic::Player;
use crate::types::{Method, Request, Response};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Connector {
    /// IP:Port
    pub server: String,
    pub username: String,
    password: String,
    is_conn: bool,
}

impl Connector {
    /// `server` - IP:PORT
    pub fn new(server: String) -> Result<Self, String> {
        let con = Self {
            server: server.trim().to_string(),
            username: String::new(),
            password: String::new(),
            is_conn: false,
        };

        let ping_res = con.request(Request::new(Method::Ping, None));
        if ping_res.successful {
            Ok(con)
        } else {
            Err(ping_res.message)
        }
    }

    pub fn auth(&mut self, username: String, password: String) {
        self.username = username.trim().to_string();
        self.password = password.trim().to_string();
    }

    pub fn connect(&mut self) -> Response {
        let resp = self.request(Request::new(
            Method::Connect((self.username.clone(), self.password.clone())),
            None,
        ));
        if resp.successful {
            self.is_conn = true;
        }
        resp
    }

    pub fn get_stats(&self) -> Result<Player, String> {
        let resp = self.request(Request::new(
            Method::GetStats,
            Some(self.username.to_string()),
        ));
        if resp.successful {
            Ok(serde_json::from_str::<Player>(&resp.message).unwrap())
        } else {
            Err(resp.message)
        }
    }

    fn check_connection(&self, method: &Method) -> bool {
        match method {
            Method::Connect(_) => true,
            Method::Ping => true,
            _ => false,
        }
    }

    fn request(&self, request: Request) -> Response {
        if self.check_connection(&request.body) || self.is_conn {
            let resp = self.base_request(serde_json::to_string(&request.clone()).unwrap());
            match resp {
                Ok(r) => serde_json::from_str(&r).unwrap(),
                Err(e) => Response {
                    successful: false,
                    message: format!("Помилка при спробі зв'язку з сервером: {}", e),
                },
            }
        } else {
            Response {
                successful: false,
                message: String::from("Ви не підключені до серверу!"),
            }
        }
    }

    fn base_request(&self, data: String) -> Result<String, String> {
        match TcpStream::connect(self.server.clone()) {
            Ok(mut stream) => {
                stream.write(data.as_bytes()).unwrap();
                let mut buf = vec![];
                loop {
                    let mut b = vec![0; 64];
                    match stream.read(&mut b) {
                        Ok(n) if n == 0 => return Ok(String::from_utf8(buf).unwrap()),
                        Ok(n) => {
                            buf.extend(&b[0..n]);
                            if n < 64 {
                                return Ok(String::from_utf8(buf).unwrap());
                            }
                        }
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    }
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
