

use    std::io ;

struct Room {
    name: String,
    is_end_room: bool,
}


struct Point {
    x: u8,
    y: u8,
}


fn main() {
    println!("WELCOME PLAYER !");
    println!("YOU HAVE TO FIND THE END ROOM !");

    let mut coord =  Point{x: 0, y:0};

    let max_x: u8 = 4;
    let max_y: u8 = 4;

    loop{
    let mut input = String::new(); // A mutable String to hold the user input
    println!("Please enter your direction :");
    
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim()); // Trim removes trailing newline
    
    println!("{}, {}", coord.x, coord.y); // Trim removes trailing newline

    let direction = input.trim(); // On stocke l'input nettoyé

        // On utilise 'match' au lieu de faire plein de 'if'
        match direction {
            "right" => {
                if coord.x < max_x {
                    coord.x += 1;
                } else {
                    println!("Wall! You can't go further right.");
                }
            }
            "left" => {
                if coord.x > 0 { // Empêche de passer en dessous de 0 (ce qui ferait planter un u8)
                    coord.x -= 1;
                } else {
                    println!("Wall! You can't go further left.");
                }
            }
            "up" => {
                if coord.y < max_y {
                    coord.y += 1;
                } else {
                    println!("Wall! You can't go further up.");
                }
            }
            "down" => {
                if coord.y > 0 {
                    coord.y -= 1;
                } else {
                    println!("Wall! You can't go further down.");
                }
            }
            "quit" => {
                println!("Goodbye!");
                break; // Quitte la boucle loop
            }
            _ => { // '_ =>' gère tous les autres mots (erreurs de frappe, etc.)
                println!("I don't understand that command.");
            }
        }

}
}
