//! Text-based adventure game "NotAPrince".

use colored::*;
use rand::SeedableRng;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::collections::BTreeMap;
use std::io::{self, Write};

/// The game ID of the starting room. Always fixed regardless of the seed.
const STARTING_ROOM_ID: i64 = 1000;
/// ID of the winning room.
const FINAL_ROOM_ID: i64 = 9999;

/// Items are either carryable can be picked up by the player
/// or non-carryable because it's too large or dangerous to move.
/// a hint towards unlocking a significant room.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Sword,
    BigBook,
    Potion,
    Demon,
    Toilet,
    Dragon,
    Duckiebot,
    ThroneScale,
    PurplePotion,
    ToiletPaper,
    MirrorShard,
    GameTicket,
    RedButton,
    DuckieWhistle,
    BedroomKey,
    BathroomSoap,
    DemonClaw,
}

impl Item {
    /// Returns the display name of the item.
    pub fn name(&self) -> &str {
        match self {
            Item::Sword => "Sword",
            Item::BigBook => "Secret big book",
            Item::Potion => "Strange potion",
            Item::Demon => "Demon",
            Item::Toilet => "Rupert the third emperor, the toilets that talks!",
            Item::Dragon => "A sleepy dragon",
            Item::Duckiebot => "A duck that drives its special vehicle",
            Item::ThroneScale => "Glittering dragon scale",
            Item::PurplePotion => "Purple potion vial",
            Item::ToiletPaper => "Roll of royal toilet paper",
            Item::MirrorShard => "Shard of a broken mirror",
            Item::GameTicket => "Ticket stub from a game show",
            Item::RedButton => "Loose red button",
            Item::DuckieWhistle => "Small duck-shaped whistle",
            Item::BedroomKey => "Brass bedroom key",
            Item::BathroomSoap => "Bar of lavender soap",
            Item::DemonClaw => "Blackened demon claw",
        }
    }

    /// Returns 'true' if this item can be picked up and added to the player's inventory.
    ///
    /// Large or dangerous entities (Demon, Toilet, Dragon) cannot be carried.

    pub fn carry_able(&self) -> bool {
        match self {
            Item::Sword
            | Item::BigBook
            | Item::Potion
            | Item::Duckiebot
            | Item::ThroneScale
            | Item::PurplePotion
            | Item::ToiletPaper
            | Item::MirrorShard
            | Item::GameTicket
            | Item::RedButton
            | Item::DuckieWhistle
            | Item::BedroomKey
            | Item::BathroomSoap
            | Item::DemonClaw => true,
            Item::Demon | Item::Toilet | Item::Dragon => false,
        }
    }

    // Returns the special word revealed by this item when picked up, if any.
    ///
    /// Each clue item is linked to a specific index in the 'special_words' list.
    /// When a player picks up this item, they learn the word needed to unlock
    /// Returns `None` for items that do not reveal a word.
    ///
    /// # Arguments
    /// * 'special_words' : The list of special words drawn at game initialization.

    pub fn revealed_word(&self, special_words: &[String]) -> Option<String> {
        match self {
            Item::ThroneScale => Some(special_words.get(0)?.clone()),
            Item::BedroomKey => Some(special_words.get(1)?.clone()),
            Item::BathroomSoap => Some(special_words.get(2)?.clone()),
            Item::DemonClaw => Some(special_words.get(3)?.clone()),
            Item::PurplePotion => Some(special_words.get(4)?.clone()),
            Item::ToiletPaper => Some(special_words.get(5)?.clone()),
            Item::MirrorShard => Some(special_words.get(6)?.clone()),
            Item::GameTicket => Some(special_words.get(7)?.clone()),
            Item::RedButton => Some(special_words.get(8)?.clone()),
            Item::DuckieWhistle => Some(special_words.get(9)?.clone()),
            _ => None,
        }
    }
}
/// Player choice in a room with explicit options.
///
/// When a room has choices, the player must type one of the commands
/// instead of using the standard `next` command.
#[derive(Debug, Clone)]
pub struct Choice {
    /// The command the player must type to select this option.
    pub command: String,
    /// A description of what this choice does, shown to the player.
    pub description: String,
    /// The 'id_game' of the room this choice leads to.
    /// A value of '-1' means the player will be prompted for a seed word.
    pub target_room: i64,
}

/// Game room containing description, items, and choices.
/// Rooms can be either hardcoded special rooms
/// or procedurally generated rooms, created on the fly from a seed word.
#[derive(Debug, Clone)]
pub struct Room {
    /// Unique identifier for this room within the game map.
    pub id_game: i64,
    /// Short name displayed as the room header.
    pub name: String,
    /// Full description shown when the player enters the room.
    pub description: String,
    /// Items currently present on the floor of this room.
    pub items: Vec<Item>,
    /// Special choices available to the player in this room.
    /// If empty, the player uses standard navigation commands.
    pub choices: Vec<Choice>,
}

impl Room {
    /// Creates a new empty room with the given game ID.
    ///
    /// If no ID is provided, defaults to '-1'.
    ///
    /// # Arguments
    /// * 'game_id' : Optional game ID to assign to this room.
    pub fn new(game_id: Option<i64>) -> Room {
        let resolved_id = game_id.unwrap_or(-1);
        Room {
            id_game: resolved_id,
            name: "Unknown Room".to_string(),
            description: format!("This room has no description. (game_id: {})", resolved_id),
            items: Vec::new(),
            choices: Vec::new(),
        }
    }

    /// Sets the room name.
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    /// Sets the room description.
    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}

/// The main game state, holding all rooms, player position, inventory,
/// and the special word system.
pub struct Game {
    /// Ordered history of room IDs visited by the player.
    visited_room_ids: Vec<i64>,
    /// Index into 'visited_room_ids' pointing to the player's current room.
    player_position_index: usize,
    /// Lookup table mapping each 'id_game' to its corresponding `Room`.
    /// Uses a 'BTreeMap' for deterministic ordering by key.
    room_map: BTreeMap<i64, Room>,
    /// The 10 special words drawn randomly at game start.
    /// Only these words unlock significant rooms.
    special_words: Vec<String>,
    /// Maps each special word to the 'id_game' of its associated significant room.
    special_word_to_room_id: BTreeMap<String, i64>,
    /// The player's current inventory of carried items.
    pub inventory: Vec<Item>,
    /// Pending messages to display at the end of the current game loop iteration.
    messages: Vec<String>,
    /// Auto-incrementing ID counter for procedurally generated rooms.
    /// Starts at 2001 to avoid collisions with special room IDs.
    next_procedural_id: i64,
}

impl Game {
    /// Creates a new game instance.
    pub fn new() -> Game {
        Game {
            visited_room_ids: Vec::new(),
            player_position_index: 0,
            room_map: BTreeMap::new(),
            special_words: Vec::new(),
            special_word_to_room_id: BTreeMap::new(),
            inventory: Vec::new(),
            messages: Vec::new(),
            next_procedural_id: 2001,
        }
    }

    /// Initializes the game: places the player in the starting room,
    /// draws the special words, builds all hardcoded rooms, and binds words to rooms.
    pub fn setup(&mut self) {
        self.visited_room_ids.push(STARTING_ROOM_ID);
        self.special_words = pick_special_words(10);
        self.define_special_rooms();
        self.bind_special_words();
        // debug: visible at game start, should be removed for final release
        self.messages
            .push(format!("Special words (debug): {:?}", self.special_words));
    }

    /// Binds each special word to a hardcoded significant room ID.
    ///
    /// the first word maps to room 1001,
    /// This mapping changes every run
    /// since the words are drawn randomly.

    fn bind_special_words(&mut self) {
        let special_room_ids = [1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010];
        for (index, word) in self.special_words.iter().enumerate() {
            self.special_word_to_room_id
                .insert(word.clone(), special_room_ids[index]);
        }
    }

    /// Builds and inserts all hardcoded special rooms into the game map.
    ///
    /// Special rooms use fixed IDs in the range 1000-1010.
    /// Game over rooms use IDs in the range 9001-9005.
    /// The final exit room has ID 9999.

    fn define_special_rooms(&mut self) {
        let mut start_room = Room::new(Some(STARTING_ROOM_ID));
        start_room.set_name("White Hall".to_string());
        start_room.set_description(
            "You wake up in a white hall. You need to get to the final room. A sword and an old book are lying on the ground.".to_string(),
        );
        start_room.items.push(Item::Sword);
        start_room.items.push(Item::BigBook);
        self.room_map.insert(STARTING_ROOM_ID, start_room);

        let mut throne = Room::new(Some(1001));
        throne.set_name("Throne Room".to_string());
        throne.set_description(
            "A majestic hall with a golden throne. A huge dragon sleeps beside it!".to_string(),
        );
        throne.items.push(Item::Dragon);
        self.room_map.insert(1001, throne);

        let mut bedroom = Room::new(Some(1002));
        bedroom.set_name("Bedroom".to_string());
        bedroom.set_description(
            "An empty bedroom with a double bed. A strange purple potion lies on the floor."
                .to_string(),
        );
        bedroom.items.push(Item::Potion);
        self.room_map.insert(1002, bedroom);

        let mut bathroom = Room::new(Some(1003));
        bathroom.set_name("Bathroom".to_string());
        bathroom.set_description(
            "A basic bathroom... except the golden toilets stand up and want to talk to you."
                .to_string(),
        );
        bathroom.items.push(Item::Toilet);
        self.room_map.insert(1003, bathroom);

        let mut dark = Room::new(Some(1004));
        dark.set_name("Dark Room".to_string());
        dark.set_description(
            "Total darkness. You feel a strong demonic presence. Do not talk to it.".to_string(),
        );
        dark.items.push(Item::Demon);
        dark.choices.push(Choice {
            command: "run".to_string(),
            description: "Run away, take the first visible door.".to_string(),
            target_room: -1,
        });
        dark.choices.push(Choice {
            command: "fight".to_string(),
            description: "Fight the demon!".to_string(),
            target_room: 9001,
        });
        self.room_map.insert(1004, dark);

        let mut lab = Room::new(Some(1005));
        lab.set_name("Alchemy Lab".to_string());
        lab.set_description(
            "The air is thick with colorful smoke. Shelves overflow with bubbling beakers."
                .to_string(),
        );
        lab.items.push(Item::Potion);
        self.room_map.insert(1005, lab);

        let mut prout = Room::new(Some(1006));
        prout.set_name("Prout Room".to_string());
        prout.set_description(
            "An extremely foul odor hits your nostrils. Welcome to the LEGENDARY PROUT ROOM!!"
                .to_string(),
        );
        prout.items.push(Item::Toilet);
        self.room_map.insert(1006, prout);

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
            description: "Look at your reflection... wait, it's smiling but you're not!"
                .to_string(),
            target_room: 1004,
        });
        mirror.choices.push(Choice {
            command: "contemplate".to_string(),
            description: "Contemplate the ripples. This might be a good idea...".to_string(),
            target_room: 1008,
        });
        self.room_map.insert(1007, mirror);

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
            target_room: -1,
        });
        self.room_map.insert(1008, monty);

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
            target_room: -1,
        });
        rb.choices.push(Choice {
            command: "talk".to_string(),
            description: "Talk to the button. You need someone to listen.".to_string(),
            target_room: 1009,
        });
        self.room_map.insert(1009, rb);

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
            target_room: FINAL_ROOM_ID,
        });
        self.room_map.insert(1010, db);

        self.add_game_over_room(
            9001,
            "You tried to fight a demonic entity with bare hands... It ripped your soul apart.",
        );
        self.add_game_over_room(9002, "You trusted the host and stayed with Door 1... The floor opened and you fell into a pit of spikes.");
        self.add_game_over_room(
            9003,
            "You smashed the mirror! Seven years of bad luck instantly crushed you.",
        );
        self.add_game_over_room(9004, "You pressed the button. BOOM! That was a mistake.");
        self.add_game_over_room(9005, "You followed the Duckiebots for hours, then days. You forgot who you are. You are now a duckiebot.");

        let mut exit_room = Room::new(Some(FINAL_ROOM_ID));
        exit_room.set_name("The Exit".to_string());
        exit_room.set_description(
            "You see a bright light ahead. Congratulations! You've found the exit!".to_string(),
        );
        self.room_map.insert(FINAL_ROOM_ID, exit_room);
    }

    /// Creates a game over room with the given ID and death message,
    /// then inserts it into the game map.
    ///
    /// # Arguments
    /// * 'id' : The game ID for this game over room.
    /// * 'death_reason' : A description of how the player died.
    fn add_game_over_room(&mut self, id: i64, death_reason: &str) {
        let mut room = Room::new(Some(id));
        room.set_name("GAME OVER".to_string());
        let full_description = format!("{}\n\nYou are dead. Type 'exit' to quit.", death_reason);
        room.set_description(full_description);
        self.room_map.insert(id, room);
    }

    /// Generates a procedural room from a seed word using deterministic hashing.
    ///
    /// The seed word is hashed using Rust's 'DefaultHasher' to produce a 'u64'.
    /// This value seeds a 'StdRng', which deterministically selects 1-2 items
    /// from the pool of clue items. The same word always produces the same room contents.
    ///
    /// # Arguments
    /// * 'seed' : The word entered by the player.
    fn generate_procedural_room_from_seed(&mut self, seed: &str) -> Room {
        let id = self.next_procedural_id;
        self.next_procedural_id += 1;

        let name = "Stone Chamber".to_string();
        let description = "A nondescript stone room. The walls are damp and cold. There are passages leading away.".to_string();

        // Deterministic RNG from the seed word
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut hasher);
        let hash = std::hash::Hasher::finish(&hasher);
        let mut rng = StdRng::seed_from_u64(hash);

        let item_count = rng.gen_range(1..=2);
        let possible_items = [
            Item::ThroneScale,
            Item::PurplePotion,
            Item::ToiletPaper,
            Item::MirrorShard,
            Item::GameTicket,
            Item::RedButton,
            Item::DuckieWhistle,
            Item::BedroomKey,
            Item::BathroomSoap,
            Item::DemonClaw,
        ];
        let mut items = Vec::new();
        for _ in 0..item_count {
            let idx = rng.gen_range(0..possible_items.len());
            items.push(possible_items[idx].clone());
        }

        let mut room = Room::new(Some(id));
        room.set_name(name);
        room.set_description(description);
        room.items = items;
        room
    }

    /// Prompts the player to enter a seed word, then moves them to the resulting room.
    ///
    /// If the word matches a special word, the player enters the associated
    /// significant room. Otherwise, a procedural room is generated from the word.
    fn prompt_for_seed_and_move(&mut self) {
        print!("Enter a word to open the door: ");
        io::stdout().flush().unwrap();
        let mut seed = String::new();
        io::stdin()
            .read_line(&mut seed)
            .expect("Failed to read input");
        let seed = seed.trim().to_lowercase();

        let target_id = if let Some(&room_id) = self.special_word_to_room_id.get(&seed) {
            // special word, leads to a hardcoded significant room
            room_id
        } else {
            //generate a procedural room
            let new_room = self.generate_procedural_room_from_seed(&seed);
            let new_id = new_room.id_game;
            self.room_map.insert(new_id, new_room);
            new_id
        };

        self.visited_room_ids.push(target_id);
        self.player_position_index += 1;
    }

    /// Pushes available commands for the current room into the message queue.
    ///
    /// # Arguments
    /// * 'room' : Reference to the current room.
    fn show_help(&mut self, room: &Room) {
        self.messages.push("Available commands:".to_string());
        self.messages
            .push("  - help: show this help message".to_string());
        self.messages
            .push("  - inventory / inv: show your inventory".to_string());
        self.messages.push("  - exit: quit the game".to_string());
        if room.choices.is_empty() {
            if room.items.iter().any(|i| i.carry_able()) {
                self.messages
                    .push("  - take: pick up a carryable item".to_string());
            }
            self.messages
                .push("  - next: go through an exit (you'll be asked for a seed word)".to_string());
        } else {
            self.messages.push("Room choices:".to_string());
            for choice in &room.choices {
                self.messages
                    .push(format!("  - {}: {}", choice.command, choice.description));
            }
        }
    }

    /// Pushes the player's current inventory contents into the message queue.
    fn show_inventory(&mut self) {
        if self.inventory.is_empty() {
            self.messages.push("Your inventory is empty.".to_string());
        } else {
            self.messages.push("You are carrying:".to_string());
            for item in &self.inventory {
                self.messages.push(format!("  - {}", item.name()));
            }
        }
    }

    /// Attempts to pick up the first carryable item in the current room.
    ///
    /// If the item reveals a special word, that word is shown to the player.
    ///
    /// # Arguments
    /// * 'current_id' : The game ID of the room the player is currently in.
    fn take_item(&mut self, current_id: i64) {
        if let Some(room) = self.room_map.get_mut(&current_id) {
            if let Some(index) = room.items.iter().position(|i| i.carry_able()) {
                let item = room.items.remove(index);
                self.messages
                    .push(format!("You picked up: {}", item.name()));

                if let Some(word) = item.revealed_word(&self.special_words) {
                    self.messages
                        .push(format!("The item whispers the word: '{}'", word));
                }

                self.inventory.push(item);
            } else {
                self.messages.push("Nothing to pick up here.".to_string());
            }
        }
    }

    /// Main game loop. Displays the current room, reads player input,
    /// and dispatches commands until the player wins or exits.
    pub fn play(&mut self) {
        println!("{}", "\n=== WELCOME TO NOTAPRINCE ===\n".bold());

        loop {
            let current_id = self.visited_room_ids[self.player_position_index];
            println!("{}", format!("You are in room id: {}", current_id).dimmed());

            let room = self.room_map.get(&current_id).unwrap().clone();
            println!("\n{}", format!("[ {} ]", room.name).bold().cyan());
            println!("{}", room.description);

            if !room.items.is_empty() {
                println!("\n{}", "Items on the ground:".yellow());
                for item in &room.items {
                    println!("  - {}", item.name());
                }
            }

            if !room.choices.is_empty() {
                println!("\n{}", "What do you do?".cyan());
                for choice in &room.choices {
                    println!("  '{}' -> {}", choice.command.bold(), choice.description);
                }
                println!("\n{}", "(You must pick one of the choices above)".red());
            } else {
                println!(
                    "\n{}",
                    "(Type 'next' to proceed, 'take' to pick up items)".dimmed()
                );
            }

            println!(
                "\n{}",
                "(Type 'help' for commands, 'exit' to quit)".dimmed()
            );

            if !self.inventory.is_empty() {
                println!("\n{}", "Your inventory:".magenta());
                for item in &self.inventory {
                    println!("  - {}", item.name());
                }
            }

            if !self.messages.is_empty() {
                println!("\n{}", "─".repeat(40).dimmed());
                for msg in self.messages.iter().rev().take(5) {
                    println!("{}", msg.dimmed());
                }
                self.messages.clear();
            }

            if current_id == FINAL_ROOM_ID {
                println!(
                    "\n{}",
                    "Congratulations! You have escaped the dungeon!"
                        .green()
                        .bold()
                );
                break;
            }

            print!("\n{} ", ">".bold());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let command = input.trim().to_lowercase();

            if command.is_empty() {
                continue;
            }
            if command == "exit" {
                break;
            }

            if command == "help" {
                self.show_help(&room);
                continue;
            }

            if command == "inventory" || command == "inv" {
                self.show_inventory();
                continue;
            }

            if !room.choices.is_empty() {
                if let Some(choice) = room
                    .choices
                    .iter()
                    .find(|c| c.command.to_lowercase() == command)
                {
                    let target = choice.target_room;
                    if target == -1 {
                        self.prompt_for_seed_and_move();
                    } else {
                        self.visited_room_ids.push(target);
                        self.player_position_index += 1;
                    }
                } else {
                    let choices_str = room
                        .choices
                        .iter()
                        .map(|c| c.command.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.messages
                        .push(format!("You must choose one of: {}", choices_str));
                }
            } else {
                match command.as_str() {
                    "next" => {
                        self.prompt_for_seed_and_move();
                    }
                    "take" => {
                        self.take_item(current_id);
                    }
                    _ => {
                        self.messages
                            .push(format!("Unknown command: '{}'", command));
                    }
                }
            }
        }

        println!("\n{}", "Thank you for playing!".yellow().bold());
    }
}

/// Randomly draws 'count' words from the built-in dictionary without replacement.
///
/// Uses a Fisher-Yates shuffle for unbiased random selection.
/// These words are the only ones that unlock significant rooms,
/// the player must discover them through exploration and item clues.
///
/// # Arguments
/// * 'count' : Number of words to draw. Must not exceed the dictionary size (20).
///
/// # Returns
/// Returns a `Vec<String>` containing `count` randomly selected words.
pub fn pick_special_words(count: usize) -> Vec<String> {
    let dictionary = vec![
        "lune", "forge", "cendre", "miroir", "epine", "brume", "ardoise", "seuil", "crypte",
        "voute", "maree", "eclair", "fosse", "givre", "ombre", "torche", "ronce", "cle", "pierre",
        "sang",
    ];
    let mut rng = rand::thread_rng();
    let mut picked = dictionary.clone();
    // Fisher-Yates shuffl
    for i in (1..picked.len()).rev() {
        let j = rng.gen_range(0..=i);
        picked.swap(i, j);
    }
    picked.truncate(count);
    picked.iter().map(|s| s.to_string()).collect()
}

fn main() {
    let mut game = Game::new();
    game.setup();
    game.play();
}
