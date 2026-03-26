use std::sync::atomic::{AtomicI64, Ordering};

// Class containing every possible room in the game.
// The player can generate a room using a random word which will define its layout.
static NEXT_ID_ROOM: AtomicI64 = AtomicI64::new(0);

pub struct Room {
    // The unique ID used to identify the type of room
    id_room: i64,
    // The ID of the room as a place where the player can move
    id_game: i64,
    // The next rooms this room can lead to
    next_rooms: Vec<i64>,
    //String describing the Room
    description : String,
    name : String,
    pub items: Vec<Item>, // pub pour faire un push ensuite
}




impl Room {
    pub fn new(name: &str, description: &str) -> Room {
        Room {
            id_room: NEXT_ID_ROOM.fetch_add(1, Ordering::SeqCst),
            id_game: -1,
            next_rooms: Vec::new(),
            description: String::new(),
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn set_id_game(&mut self, new_id: i64) {
        self.id_game = new_id;
    }
    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}


// list of items that will be present in the rooms

// debug: display the item
// clone : duplicate the object if needed
// partialeq : comparate two objects
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Sword,
    BigBook,
    Potion,
    Demon,
    Toilet,
    Dragon

}


// implementation of Item
impl Item {
    // A méthod to display the name of the items
    pub fn name(&self) -> &str {
        match self {
            Item::Sword => "Sword",
            Item::BigBook => " secret  big book",
            Item::Potion => " strange",
            Item::Demon => "Demon",
            Item::Toilet => " Rupert the third emperor, the toilets that  talks!",
            Item::Dragon => " A sleepy dragoon",

        }
    }

    // A method to describe the object
    pub fn look(&self) {
        match self {
            Item::Sword => {
                println!("A useful sword that might be a key.");
            }
            Item::BigBook => {
                println!("SECRET ROAD : BLALBLABLBLLBLALBLABLA");
            }
            Item::Potion => {
                println!("A bubbly purple potion, is it drinkable ?");
            }
            Item::Demon => {
                println!("Do NOT talk to the demon");
            }
              Item::Toilet => {
                println!("You are intrigued by this particular golden toilet and they CAN talk !");
            }
            Item::Dragon => {
                println!(" BIG BIG DRAGON but it is sleeping very deeply....");
            }
        }
    }
    pub fn carry_able(&self) -> bool {
        match self {
            Item::Sword | Item::BigBook | Item::Potion => true,
            Item::Demon | Item::Toilet | Item::Dragon => false,
        }
    }


}

fn main() {
let mut prison = Room::new("Prison", "An empty dungeon, nobody but you.");
    prison.items.push(Item::Sword);
    prison.items.push(Item::BigBook);

    let mut throne_room = Room::new("Throne Room", "A majestic hall with a golden throne. (there is a big big dragoon sleeping next to the throne !)");
    throne_room.items.push(Item::Dragon);

    let mut bedroom = Room::new("Bedroom", " An empty bedroom with a double bedroom, nothing particulair can be said.");
    bedroom.items.push(Item::Potion);

    let mut bathroom = Room::new("Bathroom", " A basic bathroom with toilets and  a shower. Huh, the golden toilets begin to stand, it has two arms and two legs. ( I think he wants to talk to you.)");
    bathroom.items.push(Item::Toilet);

    let mut dark_room = Room::new("Dark Room", "You can't see  anything , but you feel a demonic presence. ( do not talk to the demon)");
    dark_room.items.push(Item::Demon);

    
   let mut alchemy_lab = Room::new(
    "Alchemy Lab", 
    "The air is thick with colorful smoke. Shelves are filled with bubbling beakers and strange ingredients."
);
// On ajoute une potion dans cette salle
alchemy_lab.items.push(Item::Potion);
    
}