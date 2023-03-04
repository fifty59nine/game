use crate::logic::Server;
use dungeon_sdk::logic::*;
use rocksdb::{DBWithThreadMode, MultiThreaded, Options};

#[derive(Clone, Debug)]
pub struct Game {
    players: Vec<Player>,
    addr: String,
    db: Storage,
}

impl Game {
    pub fn new(addr: String) -> Self {
        let db = Storage::new();
        let players = db
            .get_all_with_prefix("player")
            .iter()
            .map(|json| Player::from_json(&json))
            .collect::<Vec<Player>>();
        Game { players, addr, db }
    }

    pub fn get_server(&self) -> Server {
        Server::new(self.addr.clone())
    }

    pub fn add_player(
        &mut self,
        player_data: (String, String),
        addr: String,
    ) -> Result<String, String> {
        let (player_name, mut password) = player_data;
        password = Self::to_hash(password);
        // Check on login and password
        let player_opt = self.players.iter().find(|p| p.login == player_name);
        match player_opt {
            Some(player) => {
                if player.password_hash == password {
                    Ok(format!(
                        "Ви успішно авторизувались як: {} ({})",
                        player_name, addr
                    ))
                } else {
                    Err(format!("Ви ввели невірний пароль!"))
                }
            }
            None => {
                let player = Player::new(player_name.clone(), password, addr);
                self.players.push(player.clone());
                self.db.write(
                    &format!("player_{}", player_name),
                    &serde_json::to_string(&player).unwrap(),
                );

                Ok(format!("Ви успішно зареєструвались!"))
            }
        }
    }

    pub fn get_player(&self, player_name: String) -> Option<&Player> {
        let player = self.players.iter().find(|p| p.login == player_name);
        if let Some(p) = player {
            Some(p)
        } else {
            None
        }
    }

    fn to_hash(data: String) -> String {
        let hash = ring::digest::digest(&ring::digest::SHA256, data.as_bytes());
        hex::encode(hash.as_ref())
    }
}

#[derive(Clone, Debug)]
struct Storage;
impl Storage {
    pub fn new() -> Self {
        let _ = DBWithThreadMode::<MultiThreaded>::open_default("./dungeondb").unwrap();
        Self
    }

    pub fn read(&self, key: &str) -> Result<String, ()> {
        let db = DBWithThreadMode::<MultiThreaded>::open_for_read_only(
            &Options::default(),
            "./dungeondb",
            false,
        )
        .unwrap();
        match db.get(key.as_bytes()) {
            Ok(Some(value)) => Ok(String::from_utf8(value).unwrap()),
            _ => Err(()),
        }
    }

    pub fn write(&self, key: &str, value: &str) {
        let db = DBWithThreadMode::<MultiThreaded>::open_default("./dungeondb").unwrap();
        db.put(key.as_bytes(), value.as_bytes()).unwrap()
    }

    pub fn get_all_with_prefix(&self, prefix: &str) -> Vec<String> {
        Self::get_all()
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(_, v)| v.clone())
            .collect::<Vec<_>>()
    }

    fn get_all() -> Vec<(String, String)> {
        let db = DBWithThreadMode::<MultiThreaded>::open_for_read_only(
            &Options::default(),
            "./dungeondb",
            false,
        )
        .unwrap();
        let mut it = db.raw_iterator();

        let mut all_data = Vec::new();
        it.seek_to_first();
        while it.valid() {
            let key = String::from_utf8(it.key().unwrap().to_vec()).unwrap();
            let val = String::from_utf8(it.value().unwrap().to_vec()).unwrap();
            all_data.push((key, val));
            it.next();
        }

        all_data
    }
}
