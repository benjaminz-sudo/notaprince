use rand::Rng;
use std::collections::BTreeMap;
use std::io;

///Class containing every possible room in the game. The player can generate a room using a random word which will define its layout.
pub struct Room {
    pub name: String,
    pub north: Option<u32>,
    pub south: Option<u32>,
    pub east: Option<u32>,
    pub west: Option<u32>,
}
impl Room {
    fn new(name: String) -> Room {
        Room {
            name,
            north: None,
            west: None,
            south: None,
            east: None,
        }
    }

    // Fonction pour décrire la salle
    fn get_description(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    //Création du BTreeMap
    let mut monde: BTreeMap<u32, Room> = BTreeMap::new();
}
