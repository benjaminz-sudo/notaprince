

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
    println!("yOU HAVE TO FIND THE END ROOM !");

    let  coord =  Point{x: 0, y:0};

    loop{
    let mut input = String::new(); // A mutable String to hold the user input
    println!("Please enter your name:");
    
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim()); // Trim removes trailing newline
    
    println!("{}, {}", coord.x, coord.y); // Trim removes trailing newline
    
    }
}
