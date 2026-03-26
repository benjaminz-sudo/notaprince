use rand::Rng; // usage de la crate pour utiliser le trait Rng
use std::collections::BTreeMap;
//use std::io;

///Class containing every possible room in the game. The player can generate a room using a random word which will define its layout.
pub struct Room {
    pub name: String,
    pub north: Option<u32>,
    pub south: Option<u32>,
    pub east: Option<u32>,
    pub west: Option<u32>,
}
impl Room {
    fn new(name: String) -> Room {
        Room {
            name,
            north: None,
            west: None,
            south: None,
            east: None,
        }
    }

    // Fonction pour décrire la salle, prend en paramètre une référence à la salle et retourne son nom
    fn get_description(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    //Création du BTreeMap
    let mut monde: BTreeMap<u32, Room> = BTreeMap::new();

    // Générateur de nombres aléatoires
    let mut rng = rand::thread_rng();

    // 20 salles avec IDs uniques
    while monde.len() < 20 {
        // ID entre 1 et 100
        //la méthode gen_range permet de générer un nombre aléatoire dans une plage donnée
        let id_aleatoire: u32 = rng.gen_range(1..101);

        // verification que l'ID est unique pour ne pas écraser une salle
        //contains_key : vérifier si une clé existe dans une HashMap ou BTreeMap
        if !monde.contains_key(&id_aleatoire) {
            let nom = format!("Salle mystère n°{}", id_aleatoire);
            let nouvelle_salle = Room::new(nom);
            monde.insert(id_aleatoire, nouvelle_salle);
        }
    }

    println!("\nListe des salles dans le BTreeMap :");
    for (id, salle) in &monde {
        println!(" - ID {}: {}", id, salle.get_description());
    }
}
