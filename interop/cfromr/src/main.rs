#[link (name = "badmath", kind = "static")]

extern "C" {
    fn bad_add(v1:f32, v2:f32) -> f32;
}

fn main() {
    println!("Try to sum");
    let res = unsafe {
        bad_add(9.0, 12.0)
    };

    println!("{}?? Is this right?", res);
}
