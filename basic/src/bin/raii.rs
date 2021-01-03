#![feature(box_syntax)]
#![allow(unused_variables)]

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn create_box() {
    let _function_box = box 3i32;
}

fn main() {
    let _boxed_int = box 5i32;

    {
        let _short_lived_box = box 4i32;
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    let x = ToDrop;
    println!("Made a ToDrop!");
}