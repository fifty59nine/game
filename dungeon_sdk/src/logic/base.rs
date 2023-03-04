use std::fmt::Display;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub login: String,
    pub password_hash: String,
    pub class: Class,
    pub balance: i64,
    pub hp: i64,
    pub damage: u32,
    pub inventory: Vec<Item>,
    pub data: String,
}

impl Player {
    pub fn new(login: String, password_hash: String, data: String) -> Self {
        Player {
            login,
            password_hash,
            data,
            ..Default::default()
        }
    }

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            login: Default::default(),
            password_hash: Default::default(),
            class: Class::Human,
            balance: Default::default(),
            hp: 100,
            damage: 10,
            inventory: Default::default(),
            data: Default::default(),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inv = String::new();
        for i in 0..self.inventory.len() {
            inv.push_str(&format!("{}) {}", i, self.inventory[i]));
        }
        write!(
            f,
            "{} ({:?})\nHP: {}\nDMG: {}\nBalance: ${}\n Inventory: {:#?}",
            self.login, self.class, self.hp, self.damage, self.balance, inv
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Class {
    Human,
    /// Class with additional DMG
    Warrior(u8),
    /// Class with additional HP    
    DungeonMaster(u8),
    /// Class with additional HP & DMG
    Alien(u8),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    id: u64,
    name: String,
    description: String,
    price: i64,
    additional_hp: i32,
    additional_dmg: i32,
    is_usable: bool,
}

impl Item {
    pub fn new(
        id: u64,
        name: impl Into<String>,
        desc: impl Into<String>,
        price: i64,
        add_hp: i32,
        add_dmg: i32,
        is_usable: bool,
    ) -> Self {
        Item {
            id,
            name: name.into(),
            description: desc.into(),
            price,
            additional_hp: add_hp,
            additional_dmg: add_dmg,
            is_usable,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}] - {}\nHP: +{}\nDMG: +{}\nМожна використати: {}\n--- ${}",
            self.name,
            self.id,
            self.description,
            self.additional_hp,
            self.additional_dmg,
            self.is_usable,
            self.price
        )
    }
}
