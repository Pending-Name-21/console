
use jni::JNIEnv;
use jni::sys::{jint, jobject};
use memmap2::MmapOptions;
use std::fs::File;
use std::io::{self};

fn show_map (rows: usize, cols:usize) -> io::Result<()> {
    let file = File::open("/home/fundacion/University/Fifth/SoftwareDevelopment/console/shared-memory/SharedMemory/text.txt")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    if mmap.len() < rows * cols {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "File size is smaller than expected matrix size"));
    }

    for i in 0..rows {
        for j in 0..cols {
            let index = i * cols + j;
            let c = mmap[index] as char;
            print!("{} ", c);
        }
        println!();
    }

    Ok(())
}

#[no_mangle]
pub extern fn Java_Main_notifyMove<'local>(env: JNIEnv<'local>, caller: jobject, rows: jint, cols: jint) -> () {
    show_map(rows.try_into().unwrap(), cols.try_into().unwrap());
}