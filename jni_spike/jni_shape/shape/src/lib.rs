use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::jdouble;

#[no_mangle]
pub extern "system" fn Java_Shape_calculateCircleArea(mut _env: JNIEnv, _class: JClass, radius: jdouble) -> jdouble {
    // Calculate area for Circle
    std::f64::consts::PI * radius * radius
}

#[no_mangle]
pub extern "system" fn Java_Shape_calculateCirclePerimeter(_env: JNIEnv, _class: JClass, radius: jdouble) -> jdouble {
    // Calculate perimeter for Circle
    2.0 * std::f64::consts::PI * radius
}

#[no_mangle]
pub extern "system" fn Java_Shape_calculateArea(_env: JNIEnv, _class: JClass, side: jdouble) -> jdouble {
    // Calculate area for Square
    side * side
}

#[no_mangle]
pub extern "system" fn Java_Shape_calculatePerimeter(_env: JNIEnv, _class: JClass, side: jdouble) -> jdouble {
    // Calculate perimeter for Square
    4.0 * side
}

#[no_mangle]
pub extern "system" fn Java_Shape_calculateArea1(_env: JNIEnv, _class: JClass, side1: jdouble, side2: jdouble) -> jdouble {
    // Calculate area for Rectangle
    side1 * side2
}

#[no_mangle]
pub extern "system" fn Java_Shape_calculatePerimeter1(_env: JNIEnv, _class: JClass, side1: jdouble, side2: jdouble) -> jdouble {
    // Calculate perimeter for Rectangle
    2.0 * (side1 + side2)
}

#[no_mangle]
pub extern "system" fn Java_Shape_calculateArea2(_env: JNIEnv, _class: JClass, side1: jdouble, side2: jdouble, side3: jdouble) -> jdouble {
    // Calculate area for Triangle using Heron's formula
    let s = (side1 + side2 + side3) / 2.0;
    (s * (s - side1) * (s - side2) * (s - side3)).sqrt()
}

#[no_mangle]
pub extern "system" fn Java_Shape_calculatePerimeter2(_env: JNIEnv, _class: JClass, side1: jdouble, side2: jdouble, side3: jdouble) -> jdouble {
    // Calculate perimeter for Triangle
    side1 + side2 + side3
}
