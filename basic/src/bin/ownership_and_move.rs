#![feature(box_syntax)]

fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains {}", c);
}

fn main() {
    // stack
    let x = 5u32;

    // copy
    let y = x;

    println!("x is {}, and y is {}", x, y);

    // heap pointer
    let a = box 5i32;

    println!("a contains: {}", a);

    // moved
    let b = a;

    // dropped, heap released
    destroy_box(b);

    /*
     * mutability
     */
    let immutable_box = box 5u32;

    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;

    println!("mutable_box contained {}", mutable_box);

    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}