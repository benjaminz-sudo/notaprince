use std::sync::atomic::{AtomicI64, Ordering};

// Class containing every possible room in the game.
// The player can generate a room using a random word which will define its layout.
static NEXT_ID_ROOM: AtomicI64 = AtomicI64::new(0);

pub struct Room {
    // The unique ID used to identify the type of room
    id_room: i64,
    // The ID of the room as a place where the player can move
    id_game: i64,
    // The next relative indexes this room can lead to
    next_rooms: Vec<i64>,
    //String describing the Room
    description : String,
}

impl Room {
    pub fn new() -> Room {
        Room {
            id_room: NEXT_ID_ROOM.fetch_add(1, Ordering::SeqCst),
            id_game: -1,
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


fn main() {
}