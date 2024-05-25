use jni::objects::{JObjectArray, JString, JValue};
use jni::{
    objects::{JClass, JObject},
    JNIEnv,
};
use jni::sys::jstring;

#[no_mangle]
pub extern "system" fn Java_Handler_notifyEvent<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    let output = env
        .new_string("Mouse")
        .expect("Couldn't create java string!");
    output
}

#[no_mangle]
pub extern "system" fn Java_Handler_helloFromRust(mut env: JNIEnv, _class: JClass, event: &JString) {
    println!("Hello from Rust\n")
}
