use rand::Rng; // usage de la crate pour utiliser le trait Rng
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::vec;

// Class containing every possible room in the game.
// The player can generate a room using a random word which will define its layout.
static NEXT_ID_ROOM: AtomicI64 = AtomicI64::new(0);

// Variable globale pour l'origine (Racine)
const ORIGINE_RACINE: i64 = 1000;

pub struct Room {
    // The unique ID used to identify the type of room : NEXT_ID_ROOM
    id_room: i64,
    // The ID of the room as a place where the player can move
    id_game: i64,

    parent: Option<i64>, // pour revenir en arrière, on peut stocker l'ID de la salle précédente (parent)

    // The next rooms this room can lead to
    next_rooms: Vec<i64>, //(Arbre Binaire : redirection vers une salle suivante)
}

impl Room {
    pub fn new(id_game: i64, parent: Option<i64>) -> Room {
        Room {
            id_room: NEXT_ID_ROOM.fetch_add(1, Ordering::SeqCst),
            id_game,
            parent,
            next_rooms: Vec::new(),
        }
    }

    //fonction pour ajouter une salle suivante à la salle actuelle
    pub fn set_id_game(&mut self, new_id: i64) {
        self.id_game = new_id;
    }
}

fn main() {
    //on cree le BtreeMap qui contiendra les salles du jeu
    let mut monde: BTreeMap<i64, Room> = BTreeMap::new();
    let mut rng = rand::thread_rng(); // on cree un générateur de nombres aléatoires

    //on cree un vecteur pour stocker les ids des salles à placer (avec leur parent)
    let mut ids = vec![(ORIGINE_RACINE, None)];

    // Tant qu'on n'a pas 20 salles et qu'on a des IDs à placer
    while monde.len() < 20 && !ids.is_empty() {
        // On prend le premier ID de la liste pour le traiter
        let (current_id, parent_id) = ids.remove(0);

        // Si la salle n'existe pas encore, on la crée
        if !monde.contains_key(&current_id) {
            let mut nouvelle_salle = Room::new(current_id, parent_id); // On crée une nouvelle salle avec l'ID de jeu actuel

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
                    ids.push((enfant_id, Some(current_id)));
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
