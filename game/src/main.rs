use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};

/// Identifiant de la salle de départ, séparé des salles normales
/// pour éviter les  collision avec les IDs générés 
const STARTING_ROOM_ID: i64 = 1000;

/// Compteur global auto-incrémenté pour attribuer un ID unique
/// à chaque instance de Room créée, indépendamment du thread.
static NEXT_ROOM_INSTANCE_ID: AtomicI64 = AtomicI64::new(0);

/// Représente une salle du donjon, qu'elle soit spéciale (hardcodée)
/// ou générée procéduralement depuis une seed.
pub struct Room {
    /// ID unique de cette instance de salle (son "type" ou "modèle")
    instance_id: i64,
    /// ID de la salle dans la carte du jeu (sa position)
    game_id: i64,
    /// Liste des game_id des salles accessibles depuis celle-ci
    next_room_ids: Vec<i64>,
    /// Texte affiché au joueur quand il entre dans la salle
    description: String,
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
            description: format!(
                "Cette salle n'a pas de description. (game_id: {})",
                resolved_id
            ),
        }
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
            description: self.description.clone(),
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
    let xor_value: u64 = word
        .bytes()
        .fold(0u64, |accumulator, byte| accumulator ^ (byte as u64));

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
    let index = (seed % ambiances.len() as u64) as usize;
    ambiances[index].to_string()
}



/// Calcule la distance de Levenshtein entre deux mots.
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

/// Structure principale du jeu, contenant l'état complet d'une partie :
/// les salles visitées, les layouts disponibles, et la position du joueur.
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
        }
    }

    /// Initialise la partie, crée la salle de départ, les salles spéciales,
    /// et tire aléatoirement les mots spéciaux du dictionnaire.
    pub fn setup(&mut self) {
        self.visited_room_ids.push(STARTING_ROOM_ID);
        self.special_words = pick_special_words();
        self.define_special_rooms();
    }

    /// Hardcode les salles spéciales du jeu et les insère dans la room_map.
    fn define_special_rooms(&mut self) {
        let mut start_room = Room::new(Some(STARTING_ROOM_ID));
        start_room.set_description(
            "Tu te réveilles dans une prison. Une épée et un vieux livre traînent au sol."
                .to_string(),
        );
        start_room.set_next_room_ids(vec![1, 2]);
        self.room_map.insert(STARTING_ROOM_ID, start_room);
    }

    /// Affiche les informations de la salle courante au joueur.
    fn print_current_room(&self) {
        let current_id = self.visited_room_ids[self.player_position_index];
        println!("\n--- Salle {} ---", current_id);
        if let Some(room) = self.room_map.get(&current_id) {
            println!("{}", room.description);
        } else {
            println!("[Salle inconnue]");
        }
    }

    /// Tente de faire avancer le joueur dans une salle
    /// en fonction du mot qu'il entre.
    /// Si le mot est spécial on redirige vers une salle signifiante.
    /// Sinon vers la salle générée procéduralement depuis la seed.
    fn move_with_word(&mut self, word: &str) {
        let seed = word_to_seed(word, self.room_counter);
        self.room_counter += 1;

        if is_special_word(word, &self.special_words) {
            println!("Le mot '{}' résonne dans le couloir...", word);
            // TODO: brancher vers la salle spéciale correspondante
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

    /// Boucle principale du jeu. Le joueur entre des mots pour avancer.
    /// Commandes : `exit` pour quitter, n'importe quel mot pour avancer.
    pub fn play(&mut self) {
        println!("=== Not a Prince ===");
        println!("Mots spéciaux de cette partie (debug) : {:?}", self.special_words);

        loop {
            self.print_current_room();
            println!("\nEntre un mot pour avancer (ou 'exit') :");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Erreur de lecture");

            let word = input.trim();

            if word == "exit" {
                break;
            }

            if !word.is_empty() {
                self.move_with_word(word);
            }
        }

        println!("Fin de la partie.");
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

    // Mélange et prend les 5 premiers
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