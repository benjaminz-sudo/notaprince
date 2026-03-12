use std::io;
use std::io::prelude::*;

///Class containing every possible room in the game. The player can generate a room using a random word which will define its layout.
pub struct Room{
    North:Room,
    South:Room,
    East:Room,
    West:Room,
}
impl Room{
    pub fn new()->Room{
        Room{
            North:None,
            West:None,
            South:None,
            East:None,
        }
    }
    pub fn new(rng:String) -> Room {
        return randomRoom(String);
    }

    pub fn randomRoom(rng:String){
        //TODO
        return;
    }
}
fn main() {
    println!("Hello, world!");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Input lu : {}", line.unwrap());
    }
}
