use rand::Rng; // usage de la crate pour utiliser le trait Rng
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
    pub fn new(id_game: i64) -> Room {
        Room {
            id_room: NEXT_ID_ROOM.fetch_add(1, Ordering::SeqCst),
            id_game,
            next_rooms: Vec::new(),
            description: String::new(),
        }
    }

    pub fn set_id_game(&mut self, new_id: i64) {
        self.id_game = new_id;
    }
    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}

fn main_benjamin(){
    //The physical rooms the player can move to. Designated by their id_game
    let mut game_rooms: Vec<i64> = Vec::new();
    game_rooms.push(1000);
    //The possible layouts for each room
    let mut room_layouts : Vec<Room>=Vec::new();
    //The player must get to the final room from room 0.
    let max_game_room: i64 = 10;
    //The index of the player'' position in the game_rooms Vec.
    let player_position_index:i64= 0;
    let room_test = Room::new(1);
    let room_test2 = Room::new(2);
    room_layouts.push(room_test);
    room_layouts.push(room_test2);
    println!("Test : {}",room_layouts[1].id_room);
}

fn main_melissa(){
    //on cree le BtreeMap qui contiendra les salles du jeu
    let mut monde: BTreeMap<i64, Room> = BTreeMap::new();
    let mut rng = rand::thread_rng(); // on cree un générateur de nombres aléatoires

    //on cree un vecteur pour stocker les ids des salles à placer (avec leur parent)
    let mut ids = vec![(ORIGINE_RACINE)];

    // Tant qu'on n'a pas 20 salles et qu'on a des IDs à placer
    while monde.len() < 20 && !ids.is_empty() {
        // On prend le premier ID de la liste pour le traiter
        let (current_id) = ids.remove(0);

        // Si la salle n'existe pas encore, on la crée
        if !monde.contains_key(&current_id) {
            let mut nouvelle_salle = Room::new(current_id); // On crée une nouvelle salle avec l'ID de jeu actuel

            //on genere un nombre aléatoire entre 1 et 2 pour le nombre de sorties
            let nb_sorties = rng.gen_range(1..=2);
            for _ in 0..nb_sorties {
                let ecart = rng.gen_range(1..=5);
                let signe = if rng.gen_bool(0.5) { 1 } else { -1 };
                let enfant_id = current_id + (signe * ecart);

                //  évite de boucler sur soi-même ou sur le parent
                if enfant_id != current_id
                    && Some(enfant_id) != parent_id
                    && !monde.contains_key(&enfant_id)
                {
                    nouvelle_salle.next_rooms.push(enfant_id);
                    ids.push(enfant_id);
                }
            }
            monde.insert(current_id, nouvelle_salle);
        }
    }

    // Affichage de la structure
    for (id, salle) in &monde {
        let parent_str = match salle.parent {
            Some(p) => p.to_string(),
            None => "RACINE".to_string(),
        };
        println!(
            " Salle {} | Parent (Retour): {} | Enfants (Avancer): {:?}",
            id, parent_str, salle.next_rooms
        );
    }
}
fn main() {

    main_benjamin();
    main_melissa();
}