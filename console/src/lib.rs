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

//use jni::JNIEnv;
//use jni::objects::{JObject, JClass, JString};

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

/* 
// Function to extract `Coord` from a `JObject`
fn get_coord(env: &mut JNIEnv, coord_obj: &JObject) -> Coord {
    let x = env.get_field(coord_obj, "x", "I").unwrap().i().unwrap();
    let y = env.get_field(coord_obj, "y", "I").unwrap().i().unwrap();
    Coord { x, y }
}

// Function to extract `Size` from a `JObject`
fn get_size(env: &mut JNIEnv, size_obj: &JObject) -> Size {
    let height = env.get_field(size_obj, "height", "D").unwrap().d().unwrap();
    let width = env.get_field(size_obj, "width", "D").unwrap().d().unwrap();
    Size { height, width }
}

// Function to extract `Sound` from a `JObject`
fn get_sound(env: &mut JNIEnv, sound_obj: &JObject) -> Sound {
    let file_path_obj = env.get_field(sound_obj, "filePath", "Ljava/lang/String;").unwrap().l().unwrap();
    let file_path: String = env.get_string(&JString::from(file_path_obj)).unwrap().into();
    let can_play = env.get_field(sound_obj, "canPlay", "Z").unwrap().z().unwrap();
    Sound { file_path, can_play }
}

// Function to extract `Sprite` from a `JObject`
fn get_sprite(env: &mut JNIEnv, sprite_obj: &JObject) -> Sprite {
    let position_obj = env.get_field(sprite_obj, "position", "LCoord;").unwrap().l().unwrap();
    let size_obj = env.get_field(sprite_obj, "size", "LSize;").unwrap().l().unwrap();
    let is_hidden = env.get_field(sprite_obj, "isHidden", "Z").unwrap().z().unwrap();
    
    let sound_obj = env.get_field(sprite_obj, "sound", "LSound;").unwrap().l();
    let sound = match sound_obj {
        Ok(obj) => Some(get_sound(env, &obj)),
        Err(_) => None,
    };

    Sprite {
        position: get_coord(env, &position_obj),
        size: get_size(env, &size_obj),
        is_hidden,
        sound,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_RenderHandler_handleSprite(env: JNIEnv, _: JClass, sprite_obj: JObject) {
    let mut env = env;
    let sprite = get_sprite(&mut env, &sprite_obj);
    // Handle sprite in Rust
    println!("{:?}", sprite);
}

#[no_mangle]
pub extern "system" fn Java_com_example_RenderHandler_handleSound(env: JNIEnv, _: JClass, sound_obj: JObject) {
    let mut env = env;
    let sound = get_sound(&mut env, &sound_obj);
    // Handle sound in Rust
    println!("{:?}", sound);
}
*/