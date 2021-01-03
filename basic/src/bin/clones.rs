#![feature(box_syntax)]

#[derive(Debug, Copy, Clone)]
struct Unit;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let nil = Unit;

    let copied_nil = nil;

    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(box 1, box 2);
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // println!("copy: {:?}", moved_pair);
    println!("clone: {:?}", cloned_pair);

}