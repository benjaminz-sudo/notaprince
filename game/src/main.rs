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
    // The next rooms this room can lead to
    next_rooms: Vec<i64>, //(Arbre Binaire : redirection vers une salle suivante)
}

impl Room {
    pub fn new() -> Room {
        Room {
            id_room: NEXT_ID_ROOM.fetch_add(1, Ordering::SeqCst),
            id_game: -1,
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

    //on cree un vecteur pour stocker les ids des salles à placer
    let mut ids = vec![ORIGINE_RACINE];

    // Tant qu'on n'a pas 20 salles et qu'on a des IDs à placer
    while monde.len() < 20 && !ids.is_empty() {
        // On prend le premier ID de la liste
        let current_id = ids.remove(0);

        // Si la salle n'existe pas encore, on la crée
        if !monde.contains_key(&current_id) {
            let mut nouvelle_salle = Room::new();
            nouvelle_salle.set_id_game(current_id);

            // On définit l'écart maximal, augmente avec le nombre de salles
            let ecart_max = (monde.len() as i64 / 5) + 2; // commence à 2, puis 3, jusqu'a 6

            // On génère 1 ou 2 enfants de manière aléatoire
            let nb_enfants = rng.gen_range(1..=2);
            for _ in 0..nb_enfants {
                //  ID enfant = ID parent +/- écart
                let ecart = rng.gen_range(1..=ecart_max);

                // l'enfant est à gauche (ID plus petit) ou à droite (ID plus grand)
                let signe = if rng.gen_bool(0.5) { 1 } else { -1 };

                // Calcul de l'ID de l'enfant en fonction du parent et de l'écart
                let enfant_id = current_id + (signe * ecart);

                //  Éviter les boucles (ne pas pointer vers un parent)
                if !monde.contains_key(&enfant_id) && enfant_id != ORIGINE_RACINE {
                    nouvelle_salle.next_rooms.push(enfant_id); // On ajoute l'enfant à la liste des salles suivantes de la salle actuelle
                    ids.push(enfant_id); // On ajoute l'enfant à la liste pour le traiter plus tard
                }
            }

            // On ajoute la nouvelle salle au monde avec son ID de jeu
            monde.insert(current_id, nouvelle_salle);
        }
    }

    println!("Monde généré avec {} salles :", monde.len());

    println!("structure de l'arbre :");
    for (id, salle) in &monde {
        println!(
            "Salle [ID_GAME: {}] (ID_ROOM : {}) -> Sorties : {:?}",
            id, salle.id_room, salle.next_rooms
        );
    }
}
