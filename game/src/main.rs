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

    // String giving the name of the Room
    name : String,

    //String describing the Room
    description : String,

    //Set of items in the room
    pub items : Vec<Item>, // public to make sure that the game can access it

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
            name: "Unknown Room".to_string(),
            items: Vec::new(),
        }
    }

    pub fn set_id_game(&mut self, new_id: i64) {
        self.id_game = new_id;
    }
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
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
            name: self.name.clone(),
            items: self.items.clone(),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Sword,
    BigBook,
    Potion,
    Demon,
    Toilet,
    Dragon,
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Sword => "Sword",
            Item::BigBook => "Secret big book",
            Item::Potion => "Strange potion",
            Item::Demon => "Demon",
            Item::Toilet => "Rupert the third emperor, the toilets that talks!",
            Item::Dragon => "A sleepy dragoon",
        }
    }

    pub fn look(&self) {
        match self {
            Item::Sword => println!("A useful sword that might be a key."),
            Item::BigBook => println!("SECRET ROAD : BLALBLABLBLLBLALBLABLA"),
            Item::Potion => println!("A bubbly purple potion, is it drinkable?"),
            Item::Demon => println!("Do NOT talk to the demon"),
            Item::Toilet => println!("You are intrigued by this particular golden toilet and they CAN talk!"),
            Item::Dragon => println!("BIG BIG DRAGON but it is sleeping very deeply...."),
        }
    }

    pub fn carry_able(&self) -> bool {
        match self {
            Item::Sword | Item::BigBook | Item::Potion => true,
            Item::Demon | Item::Toilet | Item::Dragon => false,
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
   fn define_layouts(&mut self) {
        // 1000 : La Prison
        let mut prison = Room::new(Some(1000));
        prison.set_name("Prison".to_string());
        prison.set_description("An empty dungeon, nobody but you.".to_string());
        prison.items.push(Item::Sword);
        prison.items.push(Item::BigBook);
        prison.set_next_rooms(vec![1001]); // Mène à la Salle du Trône

        // 1001 : Salle du Trône
        let mut throne_room = Room::new(Some(1001));
        throne_room.set_name("Throne Room".to_string());
        throne_room.set_description("A majestic hall with a golden throne. (there is a big big dragoon sleeping next to the throne !)".to_string());
        throne_room.items.push(Item::Dragon);
        throne_room.set_next_rooms(vec![1002]); // Mène à la Chambre

        // 1002 : La Chambre
        let mut bedroom = Room::new(Some(1002));
        bedroom.set_name("Bedroom".to_string());
        bedroom.set_description("An empty bedroom with a double bed, nothing particular can be said.".to_string());
        bedroom.items.push(Item::Potion);
        bedroom.set_next_rooms(vec![1003]); // Mène à la Salle de Bain

        // 1003 : Salle de Bain
        let mut bathroom = Room::new(Some(1003));
        bathroom.set_name("Bathroom".to_string());
        bathroom.set_description("A basic bathroom with toilets and a shower. Huh, the golden toilets begin to stand, it has two arms and two legs. (I think he wants to talk to you.)".to_string());
        bathroom.items.push(Item::Toilet);
        bathroom.set_next_rooms(vec![1004]); // Mène à la Dark Room

        // 1004 : Dark Room
        let mut dark_room = Room::new(Some(1004));
        dark_room.set_name("Dark Room".to_string());
        dark_room.set_description("You can't see anything, but you feel a demonic presence. (do not talk to the demon)".to_string());
        dark_room.items.push(Item::Demon);
        dark_room.set_next_rooms(vec![1005]); // Mène au Laboratoire d'Alchimie

        // 1005 : Alchemy Lab
        let mut alchemy_lab = Room::new(Some(1005));
        alchemy_lab.set_name("Alchemy Lab".to_string());
        alchemy_lab.set_description("The air is thick with colorful smoke. Shelves are filled with bubbling beakers and strange ingredients.".to_string());
        alchemy_lab.items.push(Item::Potion);
        // C'est la dernière salle (pas de next_rooms)

        // Ajout de TOUTES les salles à la liste
        self.room_layouts.push(prison);
        self.room_layouts.push(throne_room);
        self.room_layouts.push(bedroom);
        self.room_layouts.push(bathroom);
        self.room_layouts.push(dark_room);
        self.room_layouts.push(alchemy_lab);

        // Insertion dans l'arbre du jeu
        for room in &self.room_layouts {
            self.room_tree.insert(room.id_game, room.clone());
        }
    }

    // a modifier
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