use memmap2::MmapOptions;
use std::fs::File;
use std::io::{self, Read};
use jni::JNIEnv;
use jni::sys::{jint, jobject};

fn moveFrame (x: usize, y:usize) -> io::Result<()> {
    let mut file = File::open("/home/fundacion/University/Fifth/SoftwareDevelopment/SharedMemory/text.txt")?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mmap = unsafe { MmapOptions::new().map(&file)? };

    println!("{:?}", &mmap[x]);
    println!("{:?}", &mmap[y]);
    // println!("{:?}", &mmap[1]);

    Ok(())
}

#[no_mangle]
pub extern fn Java_Main_notifyMove<'local>(env: JNIEnv<'local>, caller: jobject, x: jint, y: jint) -> () {
    println!("RENDER FROM RUST");
    moveFrame(x.try_into().unwrap(), y.try_into().unwrap());
}