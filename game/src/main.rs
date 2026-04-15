use rand::Rng;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use colored::Colorize;
use std::io::{self, Write};

/// Starting room ID, separated from normal rooms to avoid ID collisions
const STARTING_ROOM_ID: i64 = 1000;

// Global counter for room instance IDs
static NEXT_ROOM_INSTANCE_ID: AtomicI64 = AtomicI64::new(0);

/// Items that the player can find in rooms.
/// Some are carryable, others are not.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Sword,
    BigBook,
    Potion,
    Demon,
    Toilet,
    Dragon,
    Duckiebot,
}

impl Item {
    /// Returns the displayable name of the item.
    pub fn name(&self) -> &str {
        match self {
            Item::Sword => "Sword",
            Item::BigBook => "Secret big book",
            Item::Potion => "Strange potion",
            Item::Demon => "Demon",
            Item::Toilet => "Rupert the third emperor, the toilets that talks!",
            Item::Dragon => "A sleepy dragon",
            Item::Duckiebot => "A duck that drives its special vehicle",
        }
    }

    /// Indicates if the item can be picked up by the player.
    pub fn carry_able(&self) -> bool {
        match self {
            Item::Sword | Item::BigBook | Item::Potion | Item::Duckiebot => true,
            Item::Demon | Item::Toilet | Item::Dragon => false,
        }
    }
}

/// Represents a choice offered to the player in special rooms.
#[derive(Debug, Clone)]
pub struct Choice {
    /// Command the player must type
    pub command: String,
    /// Description shown to the player
    pub description: String,
    /// game_id of the room this choice leads to
    pub target_room: i64,
}

/// Represents a room in the dungeon, whether special (hardcoded)
/// or procedurally generated from a seed.
pub struct Room {
    // Unique instance ID
    instance_id: i64,
    // The ID of the room as a place in the game
    id_game: i64,
    // The next room IDs this room can lead to
    next_rooms: Vec<i64>,
    // Name of the Room
    name: String,
    // Description of the Room
    description: String,
    // Set of items in the room
    pub items: Vec<Item>,
    // Choices associated with this room (mandatory choices)
    pub choices: Vec<Choice>,
}

impl Room {
    /// Creates a new room with an optional game ID.
    /// If no ID is provided, the room is considered unplaced.
    pub fn new(game_id: Option<i64>) -> Room {
        let resolved_id = game_id.unwrap_or(-1);
        Room {
            instance_id: NEXT_ROOM_INSTANCE_ID.fetch_add(1, Ordering::SeqCst),
            id_game: resolved_id,
            next_rooms: Vec::new(),
            name: "Unknown Room".to_string(),
            description: format!(
                "This room has no description. (game_id: {})",
                resolved_id
            ),
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

    /// Sets the accessible rooms from this one by their game_id.
    pub fn set_next_rooms(&mut self, next_rooms: Vec<i64>) {
        self.next_rooms = next_rooms;
    }
}

pub struct Game {
    /// History of game_ids visited by the player
    visited_room_ids: Vec<i64>,
    /// Player's current position index in visited_room_ids
    player_position_index: usize,
    /// Fast lookup game_id → Room
    room_map: BTreeMap<i64, Room>,
    /// Counter for generated rooms (anti-repetition for seeds)
    room_counter: u64,
    /// Special words picked at initialization that open meaningful rooms
    special_words: Vec<String>,
    /// Maps each special word to its meaningful room's game_id
    special_word_to_room_id: BTreeMap<String, i64>,
    /// Player's inventory
    pub inventory: Vec<Item>,
    /// Messages to display (history)
    messages: Vec<String>,
}

impl Game {
    /// Creates a new empty game, ready to be initialized via `setup()`.
    pub fn new() -> Game {
        Game {
            visited_room_ids: Vec::new(),
            player_position_index: 0,
            room_map: BTreeMap::new(),
            room_counter: 0,
            special_words: Vec::new(),
            special_word_to_room_id: BTreeMap::new(),
            inventory: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn setup(&mut self) {
        self.visited_room_ids.push(STARTING_ROOM_ID);
        self.special_words = pick_special_words();
        self.define_special_rooms();
        self.bind_special_words();
        self.messages.push(format!(
            "Special words (debug): {:?}",
            self.special_words
        ));
    }

    /// Links the 5 special words drawn to the special rooms.
    /// Each word opens a different room deterministically.
    fn bind_special_words(&mut self) {
        let special_room_ids = [1001, 1002, 1003, 1004, 1005];
        for (index, word) in self.special_words.iter().enumerate() {
            self.special_word_to_room_id
                .insert(word.clone(), special_room_ids[index]);
        }
    }

    /// Inserts hardcoded special rooms into the room_map.
    fn define_special_rooms(&mut self) {
        // Starting room - fixed, always the same regardless of seed
        let mut start_room = Room::new(Some(STARTING_ROOM_ID));
        start_room.set_name("White Hall".to_string());
        start_room.set_description(
            "You wake up in a white hall.  You need to get to the final room. A sword and an old book are lying on the ground.".to_string(),
        );
        start_room.items.push(Item::Sword);
        start_room.items.push(Item::BigBook);
        start_room.set_next_rooms(vec![1001]);
        self.room_map.insert(STARTING_ROOM_ID, start_room);

        // Throne room
        let mut throne = Room::new(Some(1001));
        throne.set_name("Throne Room".to_string());
        throne.set_description(
            "A majestic hall with a golden throne. A huge dragon sleeps beside it!".to_string(),
        );
        throne.items.push(Item::Dragon);
        throne.set_next_rooms(vec![1002]);
        self.room_map.insert(1001, throne);

        // Bedroom
        let mut bedroom = Room::new(Some(1002));
        bedroom.set_name("Bedroom".to_string());
        bedroom.set_description(
            "An empty bedroom with a double bed. A strange purple potion lies on the floor.".to_string(),
        );
        bedroom.items.push(Item::Potion);
        bedroom.set_next_rooms(vec![1003]);
        self.room_map.insert(1002, bedroom);

        // Bathroom
        let mut bathroom = Room::new(Some(1003));
        bathroom.set_name("Bathroom".to_string());
        bathroom.set_description(
            "A basic bathroom... except the golden toilets stand up and want to talk to you.".to_string(),
        );
        bathroom.items.push(Item::Toilet);
        bathroom.set_next_rooms(vec![1004]);
        self.room_map.insert(1003, bathroom);

        // Dark room with demon - mandatory choice
        let mut dark = Room::new(Some(1004));
        dark.set_name("Dark Room".to_string());
        dark.set_description(
            "Total darkness. You feel a strong demonic presence. Do not talk to it.".to_string(),
        );
        dark.items.push(Item::Demon);
        dark.choices.push(Choice {
            command: "run".to_string(),
            description: "Run away, take the first visible door.".to_string(),
            target_room: 1005,
        });
        dark.choices.push(Choice {
            command: "fight".to_string(),
            description: "Fight the demon!".to_string(),
            target_room: 9001,
        });
        self.room_map.insert(1004, dark);

        // Alchemy lab
        let mut lab = Room::new(Some(1005));
        lab.set_name("Alchemy Lab".to_string());
        lab.set_description(
            "The air is thick with colorful smoke. Shelves overflow with bubbling beakers.".to_string(),
        );
        lab.items.push(Item::Potion);
        lab.set_next_rooms(vec![1006]);
        self.room_map.insert(1005, lab);

        // Prout room
        let mut prout = Room::new(Some(1006));
        prout.set_name("Prout Room".to_string());
        prout.set_description(
            "An extremely foul odor hits your nostrils. Welcome to the LEGENDARY PROUT ROOM!!".to_string(),
        );
        prout.items.push(Item::Toilet);
        prout.set_next_rooms(vec![1007]);
        self.room_map.insert(1006, prout);

        // Mirror room
        let mut mirror = Room::new(Some(1007));
        mirror.set_name("Mirror Room".to_string());
        mirror.set_description(
            "You are surrounded by countless mirrors. On the ceiling, ground, walls, and even the doors are mirrors.".to_string(),
        );
        mirror.choices.push(Choice {
            command: "smash".to_string(),
            description: "Smash a mirror with your fist!".to_string(),
            target_room: 9003,
        });
        mirror.choices.push(Choice {
            command: "look".to_string(),
            description: "Look at your reflection... wait, it's smiling but you're not!".to_string(),
            target_room: 1004,
        });
        mirror.choices.push(Choice {
            command: "contemplate".to_string(),
            description: "Contemplate the ripples. This might be a good idea...".to_string(),
            target_room: 1008,
        });
        self.room_map.insert(1007, mirror);

        // Monty Hall room
        let mut monty = Room::new(Some(1008));
        monty.set_name("Game Show Room".to_string());
        monty.set_description(
            "You are blinded by strong lights. A massive cheering audience surrounds you. You're on stage!\n\
            The host shouts: 'Behind one door is an exit! Behind the other two... DEATH!'\n\
            You reach for Door 1. 'WAIT!' he yells, snaps his fingers, and Door 3 opens to a spike pit.\n\
            'Do you want to STAY with Door 1, or SWITCH to Door 2?'".to_string(),
        );
        monty.choices.push(Choice {
            command: "stay".to_string(),
            description: "Stay with Door 1. Trust your first instinct.".to_string(),
            target_room: 9002,
        });
        monty.choices.push(Choice {
            command: "switch".to_string(),
            description: "Switch to Door 2. Math is always TRUE!".to_string(),
            target_room: 1005,
        });
        self.room_map.insert(1008, monty);

        // Red button room
        let mut rb = Room::new(Some(1009));
        rb.set_name("Weird Red Button Room".to_string());
        rb.set_description(
            "The room is empty except for a red button in the corner. What do you do?".to_string(),
        );
        rb.choices.push(Choice {
            command: "press".to_string(),
            description: "Press the button. That's what buttons are for.".to_string(),
            target_room: 9004,
        });
        rb.choices.push(Choice {
            command: "lick".to_string(),
            description: "Lick the button. Does it taste like strawberries?".to_string(),
            target_room: 1005,
        });
        rb.choices.push(Choice {
            command: "talk".to_string(),
            description: "Talk to the button. You need someone to listen.".to_string(),
            target_room: 1009,
        });
        self.room_map.insert(1009, rb);

        // Duckiebot room
        let mut db = Room::new(Some(1010));
        db.set_name("Duckiebot Room".to_string());
        db.set_description(
            "Six small robots follow a weird-shaped lane. Ducks are driving them and following each other!".to_string(),
        );
        db.items.push(Item::Duckiebot);
        db.choices.push(Choice {
            command: "steal".to_string(),
            description: "Steal a duckiebot. It might be useful for your adventure.".to_string(),
            target_room: 1010,
        });
        db.choices.push(Choice {
            command: "follow".to_string(),
            description: "Follow the duckiebots. They seem to be having fun...".to_string(),
            target_room: 9005,
        });
        db.choices.push(Choice {
            command: "quack".to_string(),
            description: "Quack! You want to be the seventh duck.".to_string(),
            target_room: 1010,
        });
        self.room_map.insert(1010, db);

        // Additional rooms
        self.add_simple_room(1011, "Disgusting Kitchen", 
            "The smell of rotten meat hits you. Huge pots with suspicious green stew bubble on a fire.", vec![1012]);
        self.add_simple_room(1029, "Dark Cave", 
            "A damp stone cave. Water drips from the ceiling.", vec![1030]);
        self.add_simple_room(1030, "Storage Room", 
            "A small room filled with empty wooden crates.", vec![1031]);
        self.add_simple_room(1031, "Guard Post", 
            "A simple room with a single chair and a cold fireplace.", vec![1032]);
        self.add_simple_room(1032, "Well Room", 
            "A circular room with an old, dry stone well in the center.", vec![1033]);
        self.add_simple_room(1033, "Stone Corridor", 
            "A long, featureless stone corridor. It is very quiet.", vec![1034]);
        self.add_simple_room(1034, "Waiting Room", 
            "A dusty room with a few broken wooden benches.", vec![1035]);
        self.add_simple_room(1035, "Servant's Quarters", 
            "A cramped room with a tiny straw bed in the corner.", vec![1036]);
        self.add_simple_room(1036, "Empty Room", 
            "There is absolutely nothing in this room. Just bare walls.", vec![1037]);
        self.add_simple_room(1037, "Iron Door Room", 
            "A solid iron door blocks the way. It is heavily rusted.", vec![]);

        // Game over rooms
        self.add_game_over_room(9001, "You tried to fight a demonic entity with bare hands... It ripped your soul apart.");
        self.add_game_over_room(9002, "You trusted the host and stayed with Door 1... The floor opened and you fell into a pit of spikes.");
        self.add_game_over_room(9003, "You smashed the mirror! Seven years of bad luck instantly crushed you.");
        self.add_game_over_room(9004, "You pressed the button. BOOM! That was a mistake.");
        self.add_game_over_room(9005, "You followed the Duckiebots for hours, then days. You forgot who you are. You are now a duckiebot.");
    }

    /// Helper function to add simple rooms
    fn add_simple_room(&mut self, id: i64, name: &str, description: &str, next_rooms: Vec<i64>) {
        let mut room = Room::new(Some(id));
        room.set_name(name.to_string());
        room.set_description(description.to_string());
        room.set_next_rooms(next_rooms);
        self.room_map.insert(id, room);
    }

    /// Helper function to add game over rooms
    fn add_game_over_room(&mut self, id: i64, death_reason: &str) {
        let mut room = Room::new(Some(id));
        room.set_name("GAME OVER".to_string());
        let full_description = format!("{}\n\nYou are dead. Type 'exit' to quit.", death_reason);
        room.set_description(full_description);
        self.room_map.insert(id, room);
    }

    /// Attempts to move the player based on the entered word or command.
    /// Special word → meaningful special room.
    /// Choice command → follows the room's choice.
    /// Otherwise → procedural generation from seed.
    fn move_with_word(&mut self, word: &str) {
        let current_id = self.visited_room_ids[self.player_position_index];

        // Check if it's a choice from the current room first
        if let Some(room) = self.room_map.get(&current_id) {
            let target = room.choices.iter()
                .find(|choice| choice.command.to_lowercase() == word.to_lowercase())
                .map(|choice| choice.target_room);

            if let Some(target_id) = target {
                self.visited_room_ids.push(target_id);
                self.player_position_index += 1;
                return;
            }
        }

        // Help command
        if word == "help" {
            self.messages.push("Available commands:".to_string());
            self.messages.push("  - help: show this help message".to_string());
            self.messages.push("  - inventory / inv: show your inventory".to_string());
            self.messages.push("  - take: pick up a carryable item in the room".to_string());
            self.messages.push("  - next: go to the next connected room".to_string());
            self.messages.push("  - exit: quit the game".to_string());

            if let Some(room) = self.room_map.get(&current_id) {
                if !room.choices.is_empty() {
                    self.messages.push("Room choices:".to_string());
                    for choice in &room.choices {
                        self.messages.push(format!("  - {}: {}", choice.command, choice.description));
                    }
                }
            }
            return;
        }

        // Inventory command
        if word == "inventory" || word == "inv" {
            if self.inventory.is_empty() {
                self.messages.push("Your inventory is empty.".to_string());
            } else {
                self.messages.push("You are carrying:".to_string());
                for item in &self.inventory {
                    self.messages.push(format!("  - {}", item.name()));
                }
            }
            return;
        }

        // Take command
        if word == "take" {
            if let Some(room) = self.room_map.get_mut(&current_id) {
                let found = room.items.iter().position(|item| item.carry_able());
                if let Some(index) = found {
                    let picked = room.items.remove(index);
                    self.messages.push(format!("You picked up: {}", picked.name()));
                    self.inventory.push(picked);
                } else {
                    self.messages.push("Nothing to pick up here.".to_string());
                }
            }
            return;
        }

        // Next command
        if word == "next" {
            if let Some(room) = self.room_map.get(&current_id) {
                if !room.next_rooms.is_empty() {
                    self.visited_room_ids.push(room.next_rooms[0]);
                    self.player_position_index += 1;
                    return;
                }
            }
        }

        self.messages.push(format!("Unknown command: '{}'", word));
    }

    /// Main game loop with simple console output.
    pub fn play(&mut self) {
        println!("{}", "\n=== WELCOME TO NOTAPRINCE ===\n");

        loop {
            let current_id = self.visited_room_ids[self.player_position_index];
            
            println!("You are in room id : {}", current_id);

            if let Some(room) = self.room_map.get(&current_id) {
                // Display room name
                println!("\n{}", format!("[ {} ]", room.name).bold().cyan());
                
                // Display description
                println!("{}", room.description);
                
                // Display items
                if !room.items.is_empty() {
                    println!("\n{}", "Items on the ground:".yellow());
                    for item in &room.items {
                        println!("  - {}", item.name());
                    }
                }
                
                // Display choices
                if !room.choices.is_empty() {
                    println!("\n{}", "What do you do?".cyan());
                    for choice in &room.choices {
                        println!("  '{}' → {}", choice.command.bold(), choice.description);
                    }
                }
                
                // Display next rooms hint
                if !room.next_rooms.is_empty() && room.choices.is_empty() {
                    println!("\n{}", "(Type 'next' to continue)".dimmed());
                }

                println!("\n{}", "(Type 'help' to list available commands)".dimmed());
                
                // Display inventory
                if !self.inventory.is_empty() {
                    println!("\n{}", "Your inventory:".magenta());
                    for item in &self.inventory {
                        println!("  - {}", item.name());
                    }
                }
            }
            
            // Display recent messages
            if !self.messages.is_empty() {
                println!("\n{}", "─".repeat(40).dimmed());
                for msg in self.messages.iter().rev().take(5) {
                    println!("{}", msg.dimmed());
                }
                self.messages.clear();
            }
            
            // Get player input
            print!("\n{} ", ">".bold());
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let command = input.trim().to_lowercase();
            
            if command.is_empty() {
                continue;
            }
            
            if command == "exit" {
                break;
            }
            
            self.move_with_word(&command);
        }
        
        println!("\n{}", "Thank you for playing!".yellow().bold());
    }
}

/// Randomly picks 5 words from a built-in dictionary
/// to define the special words for this game.
/// These words are the only ones that open meaningful rooms.
pub fn pick_special_words() -> Vec<String> {
    let dictionary = vec![
        "lune", "forge", "cendre", "miroir", "épine",
        "brume", "ardoise", "seuil", "crypte", "voûte",
        "marée", "éclair", "fossé", "givre", "ombre",
        "torche", "ronce", "clé", "pierre", "sang",
    ];

    let mut rng = rand::thread_rng();
    let mut picked: Vec<String> = dictionary
        .iter()
        .map(|w| w.to_string())
        .collect();

    // In-house shuffle instead of importing from rand
    for i in (1..picked.len()).rev() {
        let j = rng.gen_range(0..=i);
        picked.swap(i, j);
    }

    picked.truncate(5);
    picked
}

fn main() {
    let mut game = Game::new();
    game.setup();
    game.play();
}