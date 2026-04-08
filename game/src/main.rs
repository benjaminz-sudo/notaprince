use rand::Rng;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::vec;


use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use kira::sound::static_sound::StaticSoundData;


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

    pub choices: Vec<Choice>, // some rooms are associated to a choice that the player MUST make

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
            choices: Vec::new(),    
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
            choices: self.choices.clone(),      

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
    duckiebot,
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub command: String,
    pub description: String,
    pub target_room: i64,
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
            Item::duckiebot => "A duck that drives its special vehicle",

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
            Item::duckiebot => println!("Quack, quack ! vrooooom, vroooom !."),

        }
    }

    pub fn carry_able(&self) -> bool {
        match self {
            Item::Sword | Item::BigBook | Item::Potion | Item::duckiebot => true,
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
    // the player's inventory to store picked up items
    pub inventory: Vec<Item>,
    // audio manager
    audio_manager: AudioManager<DefaultBackend>,
}

impl Game {
    pub fn new() -> Game {
    let audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        Game {
            game_rooms: Vec::new(),
            room_layouts: Vec::new(),
            max_game_room: 30,
            player_position_index: 0,
            room_tree: BTreeMap::new(),
            inventory: Vec::new(),
            audio_manager,
        }
    }


    pub fn setup(&mut self) {
        //Starting room has id_game==1000
        self.game_rooms.push(1000);
        Self::define_layouts(self);
        
        let background_sound = StaticSoundData::from_file("fairy_lights.mp3").unwrap();
        self.audio_manager.play(background_sound).unwrap();
    }

// building the rooms
    fn define_layouts(&mut self) {
        // Calling our "workshops" to build rooms and store them
        self.room_layouts.push(Self::build_prison());
        self.room_layouts.push(Self::build_throne_room());
        self.room_layouts.push(Self::build_bedroom());
        self.room_layouts.push(Self::build_bathroom());
        self.room_layouts.push(Self::build_dark_room());
        self.room_layouts.push(Self::build_alchemy_lab());
        self.room_layouts.push(Self::build_proute_room());
        self.room_layouts.push(Self::build_mirror_room());
        self.room_layouts.push(Self::build_monty_hall_room());
        self.room_layouts.push(Self::build_red_button_room());
        self.room_layouts.push(Self::build_duckiebot_room());
        self.room_layouts.push(Self::build_cave());
        self.room_layouts.push(Self::build_storage_room());
        self.room_layouts.push(Self::build_guard_post());
        self.room_layouts.push(Self::build_well_room());
        self.room_layouts.push(Self::build_stone_corridor());
        self.room_layouts.push(Self::build_waiting_room());
        self.room_layouts.push(Self::build_servant_quarters());
        self.room_layouts.push(Self::build_empty_room());
        self.room_layouts.push(Self::build_iron_door_room());

        // 9001 : death démon
        self.room_layouts.push(Self::build_game_over_room(9001, "You tried to fight a Demonic entity with your bare hands... It ripped your soul apart."));
        
        // 9902 :  MH death
        self.room_layouts.push(Self::build_game_over_room(9002, "You trusted the host and stayed with Door 1... The floor opened and you fell into a pit of poisoned spikes."));

        // 9003: Mirror death
        self.room_layouts.push(Self::build_game_over_room(9003, "You smashed the mirror! Seven years of bad luck instantly crushed your body."));

        // 9005 :Duckiebot death
        self.room_layouts.push(Self::build_game_over_room(9011, "You start following the Duckiebots on the lane. You follow them for hours, then days. You forgot who you are. You are just a duckiebot now."));

        // Insertion in the game tree
        for room in &self.room_layouts {
            self.room_tree.insert(room.id_game, room.clone());
        }
    }   


    // for scalabilty reasons, i chose to make function for the creation of each room
    // because of Rust and its borrow checker, we do this way : creating each room separetly instead of making a function to create the rooms, a function to add the items, then a function to add the choices and etc 
    
    fn build_prison() -> Room {
        let mut prison = Room::new(Some(1000));
        prison.set_name("Prison".to_string());
        prison.set_description("An empty dungeon, nobody but you.".to_string());
        prison.items.push(Item::Sword);
        prison.items.push(Item::BigBook);
        prison.set_next_rooms(vec![1001]);
        
        prison // Returning the finished room
    }

    fn build_throne_room() -> Room {
        let mut throne = Room::new(Some(1001));
        throne.set_name("Throne Room".to_string());
        throne.set_description("A majestic hall with a golden throne. (there is a big big dragoon sleeping next to the throne !)".to_string());
        throne.items.push(Item::Dragon);
        throne.set_next_rooms(vec![1002]);
        
        throne
    }

    fn build_bedroom() -> Room {
        let mut bedroom = Room::new(Some(1002));
        bedroom.set_name("Bedroom".to_string());
        bedroom.set_description("An empty bedroom with a double bed, nothing particular can be said.".to_string());
        bedroom.items.push(Item::Potion);
        bedroom.set_next_rooms(vec![1003]);
        
        bedroom
    }

    fn build_bathroom() -> Room {
        let mut bathroom = Room::new(Some(1003));
        bathroom.set_name("Bathroom".to_string());
        bathroom.set_description("A basic bathroom with toilets and a shower. Huh, the golden toilets begin to stand, it has two arms and two legs. (I think he wants to talk to you.)".to_string());
        bathroom.items.push(Item::Toilet);
        bathroom.set_next_rooms(vec![1004]);
        
        bathroom
    }

    fn build_dark_room() -> Room {
        let mut dark = Room::new(Some(1004));
        dark.set_name("Dark Room".to_string());
        dark.set_description("You can't see anything, but you feel a demonic presence. (do not talk to the demon)".to_string());
        dark.items.push(Item::Demon);
        
        // Adding the special choices for this room
        dark.choices.push(Choice {
            command: "run".to_string(), 
            description: "Run away, take the first door you can see".to_string(),
            target_room: 1005,
        });
        dark.choices.push(Choice {
            command: "fight".to_string(),
            description: "Fight !".to_string(),
            target_room: 9001, // Game over room
        });
        
        dark
    }

    fn build_alchemy_lab() -> Room {
        let mut lab = Room::new(Some(1005));
        lab.set_name("Alchemy Lab".to_string());
        lab.set_description("The air is thick with colorful smoke. Shelves are filled with bubbling beakers and strange ingredients.".to_string());
        lab.items.push(Item::Potion);
        lab.set_next_rooms(vec![1006]);

        lab
    }

    fn build_proute_room() -> Room {
        let mut prout = Room::new(Some(1006));
        prout.set_name("Prout Room".to_string());
        prout.set_description("An extremely foul odor comes to your nostrils ? What is it ? No, impossible ?? You are in the LEGENDARY PROUT room !!".to_string());
        prout.items.push(Item::Toilet);
        prout.set_next_rooms(vec![1007]);

        prout

    }

    fn build_mirror_room() -> Room {
        let mut mirror = Room::new(Some(1007));
        mirror.set_name("Mirror Room".to_string());
        mirror.set_description("You are surrounded by countless mirrors. On the ceiling. On the ground and on the walls. Even the doors are MIRRORS. ".to_string());
        
        mirror.choices.push(Choice {
            command: "smash".to_string(), 
            description: "Smash a mirror with your fist!".to_string(),
            target_room: 9002, // Game over : 7 years of bad luck... or you just bleed to death. -> penser à faire comment le GAME OVER ??
        });

        mirror.choices.push(Choice {
            command: "look".to_string(),
            description: "Look closely at your reflection... wait, it's smiling but you are not. Might be a demonic presence after all.".to_string(),
            target_room: 1004, //
        });

        
        mirror.choices.push(Choice {
            command: "Contemplate".to_string(),
            description: "Ripples of reveries, splendid look. Might be a good idea to touch it !".to_string(),
            target_room: 1008, //
        });
        
        
        mirror
    }


   fn build_monty_hall_room() -> Room {
        let mut monty = Room::new(Some(1008));
        monty.set_name("Game Show Room".to_string());
        monty.set_description("You are blinded by strong lights. In front of you appears a massive, cheering audience. You are on stage! The host shouts, 'Behind one of these 3 doors is an exit! Behind the other two... DEATH!'\n\nYou instinctively reach for Door 1.\n'WAIT!' the host yells. He snaps his fingers and Door 3 opens, revealing a pit of spikes.\n'I'll do you a favor,' he whispers. 'Do you want to STAY with Door 1, or SWITCH to Door 2?'".to_string());
                
        monty.choices.push(Choice {
            command: "stay".to_string(), 
            description: "Stay with Door 1. You trust your first instinct.".to_string(),
            target_room: 9003, 
        });
        
        monty.choices.push(Choice {
            command: "switch".to_string(),
            description: "Switch to Door 2. Maths are always TRUE!".to_string(),
            target_room: 1005, // Ç
        });
        monty.set_next_rooms(vec![1007]);

        monty
    }

     fn build_red_button_room() -> Room {
        let mut rb = Room::new(Some(1009));
        rb.set_name("Weird Red Butto, Room".to_string());
        rb.set_description("The room is empty. Duh, you can see a red button on the corner left of the room. What do you want to do ?".to_string());
                
        rb.choices.push(Choice {
            command: "press".to_string(), 
            description: "Press the button, that is what buttons are made for.".to_string(),
            target_room: 9003, // Game Over
        });
        
        rb.choices.push(Choice {
            command: "lick".to_string(),
            description: "Lick the button, you want to know if it tastes like strawberries.".to_string(),
            target_room: 1005, // Jail
        });

        rb.choices.push(Choice {
            command: "talk".to_string(),
            description: "You feel a bit lonely and despessed and you feel like that the button is a good hearer.".to_string(),
            target_room: 1009, // Next Room
        });


        rb.set_next_rooms(vec![1009]);

        rb
    }   
    

    fn build_duckiebot_room() -> Room {
        let mut db = Room::new(Some(1010));
        db.set_name("Duckiebot Room".to_string());
        db.set_description("There are 6 small robots following a weird-shaped lane. Huh, ducks are driving those robots and they are also following each other ?".to_string());

        db.items.push(Item::duckiebot);

                
        db.choices.push(Choice {
            command: "Steal".to_string(), 
            description: "You feel like that a duckiebot might be useful for your adventure.".to_string(),  
            target_room : 1010,    
          });
        
        db.choices.push(Choice {
            command: "follow".to_string(),
            description: " Those duckiebots seem to really have fun. Somehow, you are drawn into following them. You are now following them..".to_string(),
            target_room: 9005, 
        });

        db.choices.push(Choice {
            command: "quack".to_string(),
            description: "You want to be the seventh duck and you want to quack".to_string(),
            target_room : 1010,    

        });


        db.set_next_rooms(vec![1010]);

        db
    }   
    
    fn build_kitchen() -> Room {
        let mut room = Room::new(Some(1011));
        room.set_name("Disgusting Kitchen".to_string());
        room.set_description("The smell of rotten meat hits you. There are huge pots filled with a suspicious green stew bubbling on a fire.".to_string());
        room.set_next_rooms(vec![1012]); // Mène à la Bibliothèque
        room
    }

    fn build_cave() -> Room {
        let mut room = Room::new(Some(1029));
        room.set_name("Dark Cave".to_string());
        room.set_description("A damp stone cave. Water drips from the ceiling.".to_string());
        room.set_next_rooms(vec![1030]);
        room
    }

    fn build_storage_room() -> Room {
        let mut room = Room::new(Some(1030));
        room.set_name("Storage Room".to_string());
        room.set_description("A small room filled with empty wooden crates.".to_string());
        room.set_next_rooms(vec![1031]);
        room
    }

    fn build_guard_post() -> Room {
        let mut room = Room::new(Some(1031));
        room.set_name("Guard Post".to_string());
        room.set_description("A simple room with a single chair and a cold fireplace.".to_string());
        room.set_next_rooms(vec![1032]);
        room
    }

    fn build_well_room() -> Room {
        let mut room = Room::new(Some(1032));
        room.set_name("Well Room".to_string());
        room.set_description("A circular room with an old, dry stone well in the center.".to_string());
        room.set_next_rooms(vec![1033]);
        room
    }

    fn build_stone_corridor() -> Room {
        let mut room = Room::new(Some(1033));
        room.set_name("Stone Corridor".to_string());
        room.set_description("A long, featureless stone corridor. It is very quiet.".to_string());
        room.set_next_rooms(vec![1034]);
        room
    }

    fn build_waiting_room() -> Room {
        let mut room = Room::new(Some(1034));
        room.set_name("Waiting Room".to_string());
        room.set_description("A dusty room with a few broken wooden benches.".to_string());
        room.set_next_rooms(vec![1035]);
        room
    }

    fn build_servant_quarters() -> Room {
        let mut room = Room::new(Some(1035));
        room.set_name("Servant's Quarters".to_string());
        room.set_description("A cramped room with a tiny straw bed in the corner.".to_string());
        room.set_next_rooms(vec![1036]);
        room
    }

    fn build_empty_room() -> Room {
        let mut room = Room::new(Some(1036));
        room.set_name("Empty Room".to_string());
        room.set_description("There is absolutely nothing in this room. Just bare walls.".to_string());
        room.set_next_rooms(vec![1037]);
        room
    }

    fn build_iron_door_room() -> Room {
        let mut room = Room::new(Some(1037));
        room.set_name("Iron Door Room".to_string());
        room.set_description("A solid iron door blocks the way. It is heavily rusted.".to_string());
        // Pas de set_next_rooms ici, c'est la fin temporaire de ce chemin
        room
    }


    fn build_game_over_room(id_game: i64, death_reason: &str) -> Room {
        let mut game_over = Room::new(Some(id_game));
        game_over.set_name(" GAME OVER ".to_string());
        
        let full_description = format!("{}\n\nYou are dead. Type 'exit' to close the game and cry.", death_reason);
        game_over.set_description(full_description);
        
        // No items, no choices, no next_rooms. The player is dead!
        game_over
    }

    pub fn play(&mut self) {
        loop {
            self.print_room_info();
            println!("What do you want to do? ('next' to advance, 'take' to grab items, 'inventory' to check bag, 'exit' to leave) :");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let command = input.trim().to_lowercase();

            if command == "exit" {
                break;
            }

            // === Checking inventory ===
            if command == "inventory" || command == "inv" {
                if self.inventory.is_empty() {
                    println!(" Your inventory is empty??.");
                } else {
                    println!("You are carrying:");
                    for item in &self.inventory {
                        println!("    - {}", item.name());
                    }
                }
                continue; 
            }

            // Picking up items
            if command == "take" {
                let current_room_id = self.game_rooms[self.player_position_index as usize];
                let current_room = self.room_tree.get_mut(&current_room_id).unwrap();
                
                let mut found_index = None;
                
                // Find the FIRST carryable item
                // a modifier
                for (index, item) in current_room.items.iter().enumerate() {
                    if item.carry_able() {
                        found_index = Some(index);
                        break; 
                    }
                }

                if let Some(index) = found_index {
                    let picked_item = current_room.items.remove(index);
                    println!(" You picked up: {}", picked_item.name());
                    self.inventory.push(picked_item);
                } else {
                    println!(" You can NOT pick up this item, it might be  too heavy OR you lack of strength ...");
                }
                continue; 
            }

            let current_room_id = self.game_rooms[self.player_position_index as usize];
            let current_room = self.room_tree.get(&current_room_id).unwrap();
            let mut moved = false;

            // Case 1: Classic "next" command
            if command == "next" && !current_room.next_rooms.is_empty() {
                self.game_rooms.push(current_room.next_rooms[0]);
                self.player_position_index += 1;
                moved = true;
            } 
            // Case 2: Special choice command
            else {
                for choice in &current_room.choices {
                    if command == choice.command {
                        self.game_rooms.push(choice.target_room);
                        self.player_position_index += 1;
                        moved = true;
                        break;
                    }
                }
            }

            // If the input doesn't match anything
            if !moved {
                println!(" You do not follow the script. Unvalid action. Please refrain from doing this and do a valid action :)");
            }
        }
        println!("Exiting.");
    }

fn print_room_info(&self) {
        let current_room_id = self.game_rooms[self.player_position_index as usize];
        let current_room = self.room_tree.get(&current_room_id).unwrap();

        println!(" [{}]", current_room.name);
        println!("{}", current_room.description);
        
        // Displaying the items on the ground
        if !current_room.items.is_empty() {
            println!("\nOn the ground, you can see:");
            for item in &current_room.items {
                println!("- {}", item.name());
            }
        }

        // Displaying the possible choices
        if !current_room.choices.is_empty() {
            println!("\nYour special choices are:");
            for choice in &current_room.choices {
                println!("Type '{}' to : {}", choice.command, choice.description);
            }
        }
    }

}
fn main() {

    let mut game = Game::new();

    game.setup();    

    game.play();

}

