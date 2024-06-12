extern crate jni;

use jni::JNIEnv;
use jni::objects::{JObject, JClass, JString};

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Size {
    height: f64,
    width: f64,
}

#[derive(Debug)]
struct Sound {
    file_path: String,
    can_play: bool,
}

#[derive(Debug)]
struct Sprite {
    position: Coord,
    size: Size,
    is_hidden: bool,
    sound: Option<Sound>,
}

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