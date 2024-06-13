extern crate jni;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod frame;
pub mod serialization;
pub mod socket_client;

use crate::frame::{Coord, Size, Sprite, Sound};
use crate::serialization::serialize_to_json;
use crate::socket_client::send_data_to_server;

// Simulacion de datos
fn simulate_data_reception() -> Sprite {
    let position = Coord { x: 10, y: 20 };
    let size = Size { height: 64.0, width: 64.0 };
    let sound = Sound { file_path: String::from("/path/to/sound.wav"), can_play: true };
    let sprite = Sprite {
        position,
        size,
        is_hidden: false,
        sound: Some(sound),
    };

    sprite
}

pub fn run_client() {
    let sprite = simulate_data_reception();
    let json_data = serialize_to_json(&sprite);
    let server_address = "127.0.0.1:8080";
    send_data_to_server(&json_data, server_address);
}
