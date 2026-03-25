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

    fn generateEmptyNeighbours(&mut self) {
        self.north = Some(Box::new(Room {
            name: String::from("Salle N"),
            ..Room::new() // fill remaining fields with defaults
        }));
        self.east = Some(Box::new(Room {
            name: String::from("Salle E"),
            ..Room::new()
        }));
        self.south = Some(Box::new(Room {
            name: String::from("Salle S"),
            ..Room::new()
        }));
        self.west = Some(Box::new(Room {
            name: String::from("Salle W"),
            ..Room::new()
        }));
    }

    fn generate(rng: String) -> Room {
        return Self::randomRoom(rng);
    }

    fn randomRoom(_rng: String) -> Room {
        return Room::new();
        //TODO
    }

    fn printRooms(&self) {
        println!("Nearest rooms : \n");
        println!(
            "- north room : {}",
            self.north.as_ref().expect("Room N exists").getDescription()
        );
        println!(
            "- south room : {}",
            self.south.as_ref().expect("Room S exists").getDescription()
        );
        println!(
            "- east room : {}",
            self.east.as_ref().expect("Room E exists").getDescription()
        );
        println!(
            "- west room : {}",
            self.west.as_ref().expect("Room W exists").getDescription()
        );
    }

    fn getDescription(&self) -> String {
        return self.name.clone();
    }
}
fn main() {
    println!("Hello, world!");
    /* let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Input lu : {}", line.unwrap());
    } */
    let mut room0 = Room::new();
    room0.generateEmptyNeighbours();
    room0.printRooms();
}
