use rand::Rng;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use colored::Colorize;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

/// Identifiant de la salle de départ, séparé des salles normales
/// pour éviter les collisions avec les IDs générés
const STARTING_ROOM_ID: i64 = 1000;

// on commence à 1000 pour pas écraser les IDs normaux (0, 1, 2...)
static NEXT_ROOM_INSTANCE_ID: AtomicI64 = AtomicI64::new(0);

/// Items que le joueur peut trouver dans les salles.
/// Certains sont ramassables, d'autres non.
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
    /// Retourne le nom affichable de l'item.
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

    /// Indique si l'item peut être ramassé par le joueur.
    pub fn carry_able(&self) -> bool {
        match self {
            Item::Sword | Item::BigBook | Item::Potion | Item::Duckiebot => true,
            Item::Demon | Item::Toilet | Item::Dragon => false,
        }
    }
}

/// Représente un choix proposé au joueur dans certaines salles spéciales.
#[derive(Debug, Clone)]
pub struct Choice {
    /// Commande que le joueur doit taper
    pub command: String,
    /// Description affichée au joueur
    pub description: String,
    /// game_id de la salle vers laquelle ce choix mène
    pub target_room: i64,
}

/// Représente une salle du donjon, qu'elle soit spéciale (hardcodée)
/// ou générée procéduralement depuis une seed.
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
    /// Crée une nouvelle salle avec un ID de jeu optionnel.
    /// Si aucun ID n'est fourni, la salle est considérée non placée
    pub fn new(game_id: Option<i64>) -> Room {
        let resolved_id = game_id.unwrap_or(-1);
        Room {
            instance_id: NEXT_ROOM_INSTANCE_ID.fetch_add(1, Ordering::SeqCst),
            game_id: resolved_id,
            next_room_ids: Vec::new(),
            name: "Salle inconnue".to_string(),
            description: format!(
                "Cette salle n'a pas de description. (game_id: {})",
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

    /// Définit les salles accessibles depuis celle-ci par leur game_id.
    pub fn set_next_room_ids(&mut self, next_room_ids: Vec<i64>) {
        self.next_room_ids = next_room_ids;
    }

    /// Crée une copie de cette salle nécessaire pour l'insertion en BTreeMap.
    pub fn clone(&self) -> Room {
        Room {
            instance_id: self.instance_id,
            game_id: self.game_id,
            next_room_ids: self.next_room_ids.clone(),
            name: self.name.clone(),
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
    /// Historique des game_id visités par le joueur
    visited_room_ids: Vec<i64>,
    /// Index du joueur dans visited_room_ids
    player_position_index: usize,
    /// Lookup rapide game_id → Room
    room_map: BTreeMap<i64, Room>,
    /// Compteur de salles générées (anti-répétition pour la seed)
    room_counter: u64,
    /// Mots spéciaux tirés à l'initialisation ouvrent des salles signifiantes
    special_words: Vec<String>,
    /// Relie chaque mot spécial à un game_id de salle signifiante
    special_word_to_room_id: BTreeMap<String, i64>,
    /// Inventaire du joueur
    pub inventory: Vec<Item>,
    /// Messages à afficher dans le panneau du bas (historique)
    messages: Vec<String>,
    // the player's inventory to store picked up items
    pub inventory: Vec<Item>,
    // audio manager
    audio_manager: AudioManager<DefaultBackend>,
}

impl Game {
    /// Crée une nouvelle partie vide, prête à être initialisée via `setup()`.
    pub fn new() -> Game {
    let audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        Game {
            game_rooms: Vec::new(),
            room_layouts: Vec::new(),
            max_game_room: 30,
            visited_room_ids: Vec::new(),
            player_position_index: 0,
            room_map: BTreeMap::new(),
            room_counter: 0,
            special_words: Vec::new(),
            special_word_to_room_id: BTreeMap::new(),
            inventory: Vec::new(),
            messages: Vec::new(),
            inventory: Vec::new(),
            audio_manager,
        }
    }

    pub fn setup(&mut self) {
        self.visited_room_ids.push(STARTING_ROOM_ID);
        self.special_words = pick_special_words();
        self.define_special_rooms();
        self.bind_special_words();
        self.messages.push(format!(
            "Mots spéciaux (debug) : {:?}", self.special_words
        ));
    }

    /// Relie les 5 mots spéciaux tirés aux salles de Vishnujan.
    /// Chaque mot ouvre une salle différente de façon déterministe.
    fn bind_special_words(&mut self) {
        // les game_id correspondent aux salles hardcodées de Vishnujan
        let special_room_ids = [1001, 1002, 1003, 1004, 1005];
        for (index, word) in self.special_words.iter().enumerate() {
            self.special_word_to_room_id
                .insert(word.clone(), special_room_ids[index]);
        }
    }

    /// Insère les salles spéciales hardcodées dans la room_map.
    fn define_special_rooms(&mut self) {
        // salle de départ fixe, toujours la même peu importe la seed
        let mut start_room = Room::new(Some(STARTING_ROOM_ID));
        start_room.set_name("Prison".to_string());
        start_room.set_description(
            "Tu te réveilles dans une prison. Une épée et un vieux livre traînent au sol.".to_string(),
        );
        start_room.items.push(Item::Sword);
        start_room.items.push(Item::BigBook);
        self.room_map.insert(STARTING_ROOM_ID, start_room);

        // salle du trône
        let mut throne = Room::new(Some(1001));
        throne.set_name("Throne Room".to_string());
        throne.set_description(
            "Une salle majestueuse avec un trône doré. Un énorme dragon dort juste à côté !".to_string(),
        );
        throne.items.push(Item::Dragon);
        self.room_map.insert(1001, throne);

        // chambre
        let mut bedroom = Room::new(Some(1002));
        bedroom.set_name("Bedroom".to_string());
        bedroom.set_description(
            "Une chambre vide avec un lit double. Une étrange potion violette traîne au sol.".to_string(),
        );
        bedroom.items.push(Item::Potion);
        self.room_map.insert(1002, bedroom);

        // salle de bain
        let mut bathroom = Room::new(Some(1003));
        bathroom.set_name("Bathroom".to_string());
        bathroom.set_description(
            "Une salle de bain basique... sauf que les toilettes dorées se lèvent et veulent te parler.".to_string(),
        );
        bathroom.items.push(Item::Toilet);
        self.room_map.insert(1003, bathroom);

        // salle sombre avec démon — choix obligatoire
        let mut dark = Room::new(Some(1004));
        dark.set_name("Dark Room".to_string());
        dark.set_description(
            "L'obscurité totale. Tu ressens une forte présence démoniaque. Ne lui parle pas.".to_string(),
        );
        dark.items.push(Item::Demon);
        dark.choices.push(Choice {
            command: "run".to_string(),
            description: "Fuir, prendre la première porte visible.".to_string(),
            target_room: 1005,
        });
        dark.choices.push(Choice {
            command: "fight".to_string(),
            description: "Se battre contre le démon !".to_string(),
            target_room: 9001,
        });
        self.room_map.insert(1004, dark);

        // labo d'alchimie
        let mut lab = Room::new(Some(1005));
        lab.set_name("Alchemy Lab".to_string());
        lab.set_description(
            "L'air est épais de fumée colorée. Les étagères débordent de béchers bouillonnants.".to_string(),
        );
        lab.items.push(Item::Potion);
        self.room_map.insert(1005, lab);

        // game over démon
        let mut game_over_demon = Room::new(Some(9001));
        game_over_demon.set_name("GAME OVER".to_string());
        game_over_demon.set_description(
            "Tu as essayé de combattre une entité démoniaque à mains nues... Elle a déchiré ton âme.\n\nTu es mort. Tape 'exit' pour quitter.".to_string(),
        );
        self.room_map.insert(9001, game_over_demon);
    }

    /// Tente de faire avancer le joueur selon le mot ou la commande entrée.
    /// Mot spécial → salle signifiante de Vishnujan.
    /// Commande de choix → suit le choix de la salle actuelle.
    /// Sinon → génération procédurale depuis la seed.
    fn move_with_word(&mut self, word: &str) {
        let current_id = self.visited_room_ids[self.player_position_index];

        // vérifie d'abord si c'est un choix de la salle actuelle
        if let Some(room) = self.room_map.get(&current_id) {
            let target = room.choices.iter()
                .find(|choice| choice.command == word)
                .map(|choice| choice.target_room);

            if let Some(target_id) = target {
                self.visited_room_ids.push(target_id);
                self.player_position_index += 1;
                return;
            }
        }

        // commande inventaire
        if word == "inventory" || word == "inv" {
            if self.inventory.is_empty() {
                self.messages.push("Ton inventaire est vide.".to_string());
            } else {
                self.messages.push("Tu portes :".to_string());
                for item in &self.inventory.clone() {
                    self.messages.push(format!("  - {}", item.name()));
                }
            }
            return;
        }

        // commande take
        if word == "take" {
            if let Some(room) = self.room_map.get_mut(&current_id) {
                let found = room.items.iter().position(|item| item.carry_able());
                if let Some(index) = found {
                    let picked = room.items.remove(index);
                    self.messages.push(format!("Tu ramasses : {}", picked.name()));
                    self.inventory.push(picked);
                } else {
                    self.messages.push("Rien de ramassable ici.".to_string());
                }
            }
            return;
        }

        let seed = word_to_seed(word, self.room_counter);
        self.room_counter += 1;

        // indice si le joueur est proche d'un mot spécial sans le trouver
        if let Some(hint) = check_proximity_hint(word, &self.special_words) {
            self.messages.push(hint.to_string());
        }

        if is_special_word(word, &self.special_words) {
            // on récupère le game_id associé à ce mot spécial
            if let Some(&target_id) = self.special_word_to_room_id.get(word) {
                self.messages.push(format!("Le mot '{}' résonne dans le couloir...", word));
                self.visited_room_ids.push(target_id);
                self.player_position_index += 1;
            }
        } else {
            let description = generate_room_description(seed);
            let new_id = self.room_counter as i64;
            let mut new_room = Room::new(Some(new_id));
            new_room.set_description(description);
            self.room_map.insert(new_id, new_room);
            self.visited_room_ids.push(new_id);
            self.player_position_index += 1;
        }
    }

    /// Boucle principale du jeu avec interface ratatui.
    /// Affiche la salle, l'inventaire et l'historique des messages.
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
        // initialisation du terminal en mode raw pour ratatui
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        loop {
            let current_id = self.visited_room_ids[self.player_position_index];
            let room_name = self.room_map.get(&current_id)
                .map(|r| r.name.clone())
                .unwrap_or("???".to_string());
            let room_desc = self.room_map.get(&current_id)
                .map(|r| r.description.clone())
                .unwrap_or_default();
            let items_text: Vec<String> = self.room_map.get(&current_id)
                .map(|r| r.items.iter().map(|i| format!("  - {}", i.name())).collect())
                .unwrap_or_default();
            let choices_text: Vec<String> = self.room_map.get(&current_id)
                .map(|r| r.choices.iter().map(|c| format!("  '{}' → {}", c.command, c.description)).collect())
                .unwrap_or_default();
            let inv_text: Vec<String> = self.inventory.iter()
                .map(|i| format!("  - {}", i.name()))
                .collect();
            let messages_clone = self.messages.clone();

            // dessin de l'interface ratatui
            terminal.draw(|frame| {
                let area = frame.size();

                // découpage vertical :
                let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Percentage(65),
                    Constraint::Percentage(34),
                ])
                .split(area);

                // découpage horizontal du haut : salle à gauche, inventaire à droite
                let horizontal = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(vertical[1]);

                // panneau salle (gauche)
                let mut room_lines = vec![
                    Line::from(Span::styled(
                        room_desc.clone(),
                        Style::default().fg(Color::White),
                    )),
                ];
                if !items_text.is_empty() {
                    room_lines.push(Line::from(""));
                    room_lines.push(Line::from(Span::styled("Au sol :", Style::default().fg(Color::Yellow))));
                    for item in &items_text {
                        room_lines.push(Line::from(Span::styled(item.clone(), Style::default().fg(Color::Yellow))));
                    }
                }
                if !choices_text.is_empty() {
                    room_lines.push(Line::from(""));
                    room_lines.push(Line::from(Span::styled("Que fais-tu ?", Style::default().fg(Color::Cyan))));
                    for choice in &choices_text {
                        room_lines.push(Line::from(Span::styled(choice.clone(), Style::default().fg(Color::Cyan))));
                    }
                }

                let room_widget = Paragraph::new(room_lines)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title(Span::styled(
                            format!(" {} ", room_name),
                            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                        )));
                frame.render_widget(room_widget, horizontal[0]);

                // panneau inventaire (droite)
                let inv_items: Vec<ListItem> = if inv_text.is_empty() {
                    vec![ListItem::new("(vide)")]
                } else {
                    inv_text.iter().map(|i| ListItem::new(i.as_str())).collect()
                };
                let inv_widget = List::new(inv_items)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title(Span::styled(
                            " Inventaire ",
                            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                        )));
                frame.render_widget(inv_widget, horizontal[1]);

                // panneau messages en bas
                let msg_items: Vec<ListItem> = messages_clone.iter().rev().take(6)
                    .map(|m| ListItem::new(m.as_str()))
                    .collect();
                let msg_widget = List::new(msg_items)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title(Span::styled(
                            " Historique ",
                            Style::default().fg(Color::Blue),
                        )));
                frame.render_widget(msg_widget, vertical[2]);

            }).unwrap();

            // lecture de la commande du joueur en mode normal (pas raw)
            disable_raw_mode().unwrap();
            print!("\n> ");
            use std::io::Write;
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
            enable_raw_mode().unwrap();

            let word = command;
            if word == "exit" {
                break;
            }
            if !word.is_empty() {
                self.messages.push(format!("> {}", word));
                self.move_with_word(word);
            }
        }

        // on remet le terminal dans son état normal à la fin
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        println!("{}", "Fin de la partie.".yellow().bold());
    }
}

/// Tire aléatoirement 5 mots d'un dictionnaire intégré
/// pour définir les mots spéciaux de cette partie.
/// Ces mots sont les seuls à ouvrir des salles signifiantes.
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

    // shuffle maison plutôt que d'importer shuffle de rand
    for i in (1..picked.len()).rev() {
        let j = rng.gen_range(0..=i);
        picked.swap(i, j);
    }

    picked.truncate(5);
    picked
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

