extern crate jni;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod frame;
pub mod serialization;
pub mod socket_client;

use std::time::Duration;
use std::{env, thread};

use dotenv::dotenv;

use crate::frame::{Coord, Size, Sprite, Sound, Frame};
use crate::serialization::serialize_to_json;
use crate::socket_client::send_data_to_server;

// Simulación de datos
fn simulate_data_reception(x: i32, y: i32) -> Frame {
    let position = Coord { x, y };
    let size = Size { height: 64.0, width: 64.0 };
    let sound = Sound { file_path: String::from("assets/sounds/pacman_sound.wav"), can_play: true };
    let sprite = Sprite {
        position,
        size,
        is_hidden: false,
        file_path: String::from("assets/images/pacman.jpeg"),
    };

    Frame {
        sprite: Some(sprite),
        sound: Some(sound),
    }
}

pub fn run_client() {
    dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    
    let mut x = 50;
    let mut y = -100;

    for _ in 0..100 {
        let frame = simulate_data_reception(x, y);
        let json_data = serialize_to_json(&frame);
        send_data_to_server(&json_data, &server_address);

        x += 4;
        y -= 4;

        thread::sleep(Duration::from_millis(80));
    }
}
