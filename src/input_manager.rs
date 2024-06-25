use jni::objects::{JClass, JObject, JValue};
use jni::sys::jobject;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_bridge_processinputhandler_listeners_KeyboardListener_listen(
    mut env: JNIEnv,
    _class: JClass,
) -> jobject {
    let events = vec!["KeyA", "KeyB", "KeyC"];
    create_java_list(&mut env, &events)
}

#[no_mangle]
pub extern "system" fn Java_com_bridge_processinputhandler_listeners_MouseListener_listen(
    mut env: JNIEnv,
    _class: JClass,
) -> jobject {
    let events = vec!["Click", "Move", "Scroll"];
    create_java_list(&mut env, &events)
}

fn create_java_list<T: AsRef<str>>(env: &mut JNIEnv, elements: &[T]) -> jobject {
    // Encuentra la clase ArrayList en el entorno Java
    let array_list_class = env.find_class("java/util/ArrayList").expect("Failed to find ArrayList class");
    
    // Crea una nueva instancia de ArrayList
    let array_list = env.new_object(array_list_class, "()V", &[]).expect("Failed to create ArrayList instance");
        
    // Itera sobre los elementos proporcionados y los añade a la lista
    for element in elements {
        let jstring = env.new_string(element.as_ref()).expect("Failed to create Java string");
        let jstring_obj = JObject::from(jstring);

        // Llama al método 'add' en la instancia de ArrayList para añadir el nuevo string
        env.call_method(
            &array_list, 
            "add",
            "(Ljava/lang/Object;)Z", // Firma del método
            &[JValue::from(&jstring_obj)] // Argumentos del método
        ).expect("Failed to call add method");     
    }

    // Retorna el objeto raw de la lista
    array_list.into_raw()
}
