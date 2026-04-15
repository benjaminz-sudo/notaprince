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
pub struct Room {
    /// ID unique de cette instance de salle (son "type" ou "modèle")
    instance_id: i64,
    /// ID de la salle dans la carte du jeu (sa position)
    game_id: i64,
    /// Liste des game_id des salles accessibles depuis celle-ci
    next_room_ids: Vec<i64>,
    /// Nom court affiché en en-tête
    name: String,
    /// Texte affiché au joueur quand il entre dans la salle
    description: String,
    /// Items présents dans la salle
    pub items: Vec<Item>,
    /// Choix spéciaux proposés au joueur dans cette salle
    pub choices: Vec<Choice>,
}

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

    /// Remplace le nom court de la salle.
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    /// Remplace la description affichée au joueur.
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
            items: self.items.clone(),
            choices: self.choices.clone(),
        }
    }
}

/// Convertit un mot entré par le joueur en seed numérique
/// via XOR des valeurs ASCII de chaque caractère,
/// puis intègre le compteur de salle pour garantir l'unicité
/// même si le joueur répète le même mot à deux endroits différents.
///
/// # Arguments
/// * `word` - Le mot entré par le joueur
/// * `room_counter` - Le numéro de la salle actuelle (évite la répétition)
///
/// # Returns
/// Une seed `u64` unique à ce mot + cette position dans le donjon
pub fn word_to_seed(word: &str, room_counter: u64) -> u64 {
    // on additionne les valeurs ASCII de chaque lettre avec XOR
    // ex: "lune" → 108 ^ 117 ^ 110 ^ 101
    let xor_value: u64 = word
        .bytes()
        .fold(0u64, |accumulator, byte| accumulator ^ (byte as u64));

    // le grand nombre ici c'est une constante classique de LCG (générateur linéaire)
    // ça disperse bien les valeurs proches pour éviter les collisions
    xor_value
        .wrapping_mul(6364136223846793005)
        .wrapping_add(room_counter)
}

/// Vérifie si un mot appartient à la liste des mots spéciaux
/// tirés aléatoirement à l'initialisation du jeu.
/// Un mot spécial ouvre toujours une salle signifiante.
///
/// # Arguments
/// * `word` - Le mot entré par le joueur
/// * `special_words` - Les mots spéciaux de cette partie
pub fn is_special_word(word: &str, special_words: &[String]) -> bool {
    // any() s'arrête dès qu'il trouve une correspondance
    special_words.iter().any(|special| special == word)
}

/// Génère procéduralement une description de salle neutre
/// à partir d'une seed, en piochant dans des ambiances prédéfinies.
/// Deux seeds différentes → deux descriptions différentes.
///
/// # Arguments
/// * `seed` - La seed calculée depuis le mot + compteur
pub fn generate_room_description(seed: u64) -> String {
    let ambiances = [
        "Un couloir humide. L'eau suinte des murs.",
        "Une salle vide. L'écho de tes pas se perd dans l'obscurité.",
        "Des gravures étranges couvrent les murs. Rien d'utile ici.",
        "Une impasse. La poussière indique que personne n'est passé depuis longtemps.",
        "Une alcôve sombre. Tu sens une présence... mais il n'y a rien.",
    ];
    // modulo sur la seed pour rester dans les bornes du tableau
    let index = (seed % ambiances.len() as u64) as usize;
    ambiances[index].to_string()
}

/// Calcule la distance de Levenshtein entre deux mots.
/// Distance 0 = identiques, distance 1 = une lettre différente.
///
/// # Arguments
/// * `word_a` - Premier mot
/// * `word_b` - Deuxième mot
pub fn levenshtein_distance(word_a: &str, word_b: &str) -> usize {
    let chars_a: Vec<char> = word_a.chars().collect();
    let chars_b: Vec<char> = word_b.chars().collect();
    let len_a = chars_a.len();
    let len_b = chars_b.len();

    // matrice de distances, chaque case vaut un coût minimum pour
    // transformer les i premiers caractères de word_a en j premiers de word_b
    let mut matrix = vec![vec![0usize; len_b + 1]; len_a + 1];

    for i in 0..=len_a { matrix[i][0] = i; }
    for j in 0..=len_b { matrix[0][j] = j; }

    for i in 1..=len_a {
        for j in 1..=len_b {
            // si les lettres sont identiques, pas de coût supplémentaire
            let cost = if chars_a[i-1] == chars_b[j-1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i-1][j] + 1)
                .min(matrix[i][j-1] + 1)
                .min(matrix[i-1][j-1] + cost);
        }
    }
    matrix[len_a][len_b]
}

/// Vérifie si le mot est proche d'un mot spécial (distance = 1)
/// et retourne un message d'ambiance si c'est le cas.
///
/// # Arguments
/// * `word` - Le mot entré par le joueur
/// * `special_words` - Les mots spéciaux de cette partie
pub fn check_proximity_hint(word: &str, special_words: &[String]) -> Option<&'static str> {
    let hints = [
        "Quelque chose vibre dans l'air...",
        "Un frisson parcourt tes doigts.",
        "L'atmosphère change imperceptiblement.",
        "Tu sens que tu brûles...",
    ];

    let is_close = special_words
        .iter()
        .any(|special| levenshtein_distance(word, special) == 1);

    if is_close {
        // on varie le message selon la longueur du mot pour moins de répétition
        Some(hints[word.len() % hints.len()])
    } else {
        None
    }
}

/// Structure principale du jeu, contenant l'état complet d'une partie.
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
}

impl Game {
    /// Crée une nouvelle partie vide, prête à être initialisée via `setup()`.
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

    /// Initialise la partie, crée la salle de départ, les salles spéciales,
    /// et tire aléatoirement les mots spéciaux du dictionnaire.
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

                // découpage vertical : salle en haut, messages en bas
                let vertical = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .split(area);

                // découpage horizontal du haut : salle à gauche, inventaire à droite
                let horizontal = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .split(vertical[0]);

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
                frame.render_widget(msg_widget, vertical[1]);

            }).unwrap();

            // lecture de la commande du joueur en mode normal (pas raw)
            disable_raw_mode().unwrap();
            print!("\n> ");
            use std::io::Write;
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
            enable_raw_mode().unwrap();

            let word = input.trim();
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

fn main() {
    let mut game = Game::new();
    game.setup();
    game.play();
}