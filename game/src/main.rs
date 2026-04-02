use rand::Rng;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::vec;

// Variable globale pour l'origine (Racine)
const ORIGINE_RACINE: i64 = 1000;

pub struct Room {
    // The unique ID used to identify the type of room : NEXT_ID_ROOM
    id_room: i64,
    // The ID of the room as a place where the player can move
    id_game: i64,

    // The next relative indexes this room can lead to
    next_rooms: Vec<i64>,
    //String describing the Room
    description : String,

}
// Class containing every possible room in the game.
// The player can generate a room using a random word which will define its layout.
static NEXT_ID_ROOM: AtomicI64 = AtomicI64::new(0);
impl Room {
    pub fn new(id_game: Option<i64>) -> Room {
        Room {
            id_room: NEXT_ID_ROOM.fetch_add(1, Ordering::SeqCst),
            id_game: id_game.unwrap_or(-1),
            next_rooms: Vec::new(),
            description: "This room has no description here is the id_game : ".to_string()+id_game.unwrap_or(-1).to_string().as_str(),
        }
    }

    pub fn set_id_game(&mut self, new_id: i64) {
        self.id_game = new_id;
    }
    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }

    pub fn set_next_rooms(&mut self, next_rooms: Vec<i64>) {
        self.next_rooms = next_rooms;
    }

    pub fn clone(&self) -> Room {
        Room {
            id_room: self.id_room,
            id_game: self.id_game,
            next_rooms: self.next_rooms.clone(),
            description: self.description.clone(),
        }
    }
}


pub struct Game {
    //The physical rooms the player can move to. Designated by their id_game
    game_rooms: Vec<i64>,
    //The possible layouts for each room
    room_layouts: Vec<Room>,
    //The player must get to the final room from room 0.
    max_game_room: i64,
    //The index of the players' position in the game_rooms Vec.
    player_position_index: i64,
    //Arbre binaire avec pour étiquette les id_games et pour valeur les salles correspondantes.
    room_tree: BTreeMap<i64, Room>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            game_rooms: Vec::new(),
            room_layouts: Vec::new(),
            max_game_room: 10,
            player_position_index: 0,
            room_tree: BTreeMap::new(),
        }
    }

    pub fn setup(&mut self) {
        //Starting room has id_game==1000
        self.game_rooms.push(1000);
        Self::define_layouts(self);
    }

    //Fonction qui définit les différentes salles du jeu. Chaque salle a un id_room unique et un id_game qui peut être utilisé pour les différencier dans le jeu.
    fn define_layouts(&mut self){
        let mut start_room = Room::new(Some(1000));
        start_room.set_description("You are in the starting room.".to_string());
        start_room.set_next_rooms(vec![1, 2]);

        let room_test = Room::new(Some(1));
        let room_test2 = Room::new(Some(2));
        let room_tes3 = Room::new(Some(3));
        let room_test4 = Room::new(Some(4));
        
        self.room_layouts.push(start_room);
        self.room_layouts.push(room_test);
        self.room_layouts.push(room_test2);
        self.room_layouts.push(room_tes3);
        self.room_layouts.push(room_test4);

        for room in &self.room_layouts {
            self.room_tree.insert(room.id_game, room.clone());
        }
    }

    fn print_room_info(&self) {
        let current_room_id = self.game_rooms[self.player_position_index as usize];
        println!("You are in room with id_game: {}", current_room_id);
        println!("Room description: {}", self.room_tree.get(&current_room_id).unwrap().description);
    }

    pub fn play(&mut self) {
        loop {
            self.print_room_info();
            println!("Some weird text that will get printed.Awaiting input (exit to leave) : ");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim() == "exit" {
                break;
            }
            if input.trim() == "next" {
                let current_room_id = self.game_rooms[self.player_position_index as usize];
                let next_rooms = &self.room_tree.get(&current_room_id).unwrap().next_rooms;
                if !next_rooms.is_empty() {
                    let next_room_id = next_rooms[0]; // Just take the first next room for simplicity
                    self.game_rooms.push(next_room_id);
                    self.player_position_index += 1;
                } else {
                    println!("No next rooms available.");
                }
            }
        }
        println!("Exiting.");
    }
}

fn main() {

    let mut game = Game::new();

    game.setup();    

    game.play();

}